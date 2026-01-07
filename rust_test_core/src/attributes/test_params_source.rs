mod source_type;

pub use crate::attributes::test_params_source::source_type::SourceType;
use crate::attributes::common::generate_test_set;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::{parse2, ItemFn, LitStr, Type};

pub fn test_params_source(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let source: SourceType = parse2(attr).map_err(|e| {
        syn::Error::new(
            e.span(),
            format!("Expected [`rust_test::SourceType`] variant: {}", e),
        )
    })?;
    let input_fn: ItemFn =
        parse2(item).map_err(|e| syn::Error::new(e.span(), format!("Expected a function: {}", e)))?;
    let fn_name = input_fn.sig.ident.clone();

    // 1. Extract parameter type from function if not provided in attribute
    let (file_path, mut type_name): (LitStr, Option<Type>) = match source {
        SourceType::JsonFile(path, ty) => (path, ty),
    };

    if type_name.is_none() {
        if input_fn.sig.inputs.is_empty() {
            return Err(syn::Error::new_spanned(
                &input_fn.sig.inputs,
                "test generation from source requires at least one parameter",
            ));
        }

        if input_fn.sig.inputs.len() == 1 {
            if let Some(syn::FnArg::Typed(pat_type)) = input_fn.sig.inputs.first() {
                type_name = Some((*pat_type.ty).clone());
            }
        }
        // For multiple parameters, type_name remains None and generate_test_set will infer it as a tuple.
    }

    let file_path_value = file_path.value();

    // Resolve the full path
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| {
        syn::Error::new_spanned(&file_path, "CARGO_MANIFEST_DIR not set")
    })?;
    let full_path = std::path::Path::new(&manifest_dir).join(&file_path_value);
    let file_path_literal = full_path.to_str().ok_or_else(|| {
        syn::Error::new_spanned(&file_path, "Path contains invalid UTF-8")
    })?;

    let const_name = format_ident!("{}_DATA", fn_name.to_string().to_uppercase());

    // Read the file
    let file_content = std::fs::read_to_string(&full_path).map_err(|e| {
        syn::Error::new_spanned(
            &file_path,
            format!("Could not read file {}: {}", full_path.display(), e),
        )
    })?;

    let is_vec = if let Some(Type::Path(type_path)) = &type_name {
        type_path.path.segments.last().map(|s| s.ident.to_string() == "Vec").unwrap_or(false)
    } else {
        false
    };

    let type_name_opt = type_name;

    // Parse JSON and generate tests
    let tests_stream: TokenStream = match serde_json::from_str(&file_content) {
        Ok(Value::Array(array)) => {
            // If expected type is Vec, try parsing it as both list of list and just single list before throwing an error
            if is_vec {
                // Try treating each element as a test case (list of list)
                // We don't really know if it will work until runtime when we deserialize, 
                // but we can try to peek if the first element is also an array.
                // However, the requirement says "try parsing it as both... before throwing an error".
                // Since this is compile time (proc macro), we can't "try parsing" with actual data easily,
                // unless we do it here.
                let all_are_arrays = array.iter().all(|v| v.is_array());
                if all_are_arrays {
                    generate_test_set(input_fn, array, fn_name, type_name_opt)?
                } else {
                    // Treat the whole array as a single test case (single list)
                    generate_test_set(input_fn, vec![Value::Array(array)], fn_name, type_name_opt)?
                }
            } else {
                generate_test_set(input_fn, array, fn_name, type_name_opt)?
            }
        }
        Ok(single_value) => generate_test_set(input_fn, vec![single_value], fn_name, type_name_opt)?,
        Err(e) => {
            return Err(syn::Error::new_spanned(
                &file_path,
                format!("Could not parse JSON file {}: {}", file_path_value, e),
            ))
        }
    };

    let file_path_const = quote! { const #const_name: &str = include_str!(#file_path_literal); };

    // Output the const + generated tests
    Ok(quote! {
        /// --- Test data source
        #file_path_const
        #tests_stream
    })
}
