mod source_type;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use crate::__private::serde_json::Value;
use syn::parse::{Parse, ParseStream};
use syn::{parse2, ItemFn, LitStr, Path, Token, Type};

/// A source type to generate tests from.
///
/// # Variants
/// - `SourceType::JsonFile(LitStr, Type)` — pass a path to a JSON file and a type to deserialize it into.`
pub enum SourceType {
    JsonFile(LitStr, Type),
}

fn parse_as_json_file(input: ParseStream) -> syn::Result<SourceType> {
    let content;
    syn::parenthesized!(content in input);

    let file_path: LitStr = content.parse()?;
    content.parse::<Token![,]>()?;
    let type_name: Type = content.parse()?;

    Ok(SourceType::JsonFile(file_path, type_name))
}

impl Parse for SourceType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the path like `SourceType::JsonFile`
        let path: Path = input.parse()?;

        // Ensure the last segment is `JsonFile`
        let variant = path
            .segments
            .last()
            .ok_or_else(|| syn::Error::new_spanned(&path, "Expected a variant"))?
            .ident
            .to_string();

        let message = format!("Expected [`SourceType`] variant, got {}", variant);
        match variant.as_str() {
            "JsonFile" => parse_as_json_file(input),
            _ => Err(syn::Error::new_spanned(path, message)),
        }
    }
}

fn generate_test_set(
    mut input_fn: ItemFn,
    json_array: Vec<Value>,
    fn_name: Ident,
    type_name: Type,
) -> TokenStream {
    let impl_fn_name = format_ident!("{}_impl", fn_name);
    input_fn.sig.ident = impl_fn_name.clone();

    let test_functions = if json_array.len() == 1 {
        let value = &json_array[0];
        let json_str = serde_json::to_string(value).unwrap();
        let docstring = format!("Generated test {}", fn_name);

        quote! {
            #[doc = #docstring]
            #[test]
            fn #fn_name() {
                let data: #type_name = serde_json::from_str(#json_str).unwrap();
                #impl_fn_name(data);
            }
        }
    } else {
        let tests = json_array.into_iter().enumerate().map(|(i, value)| {
            let json_str = serde_json::to_string(&value).unwrap();
            let test_fn_name = format_ident!("{}_{}", fn_name, i);
            let docstring = format!("Generated test {} #{}", fn_name, i);

            quote! {
                #[doc = #docstring]
                #[test]
                fn #test_fn_name() {
                    let data: #type_name = serde_json::from_str(#json_str).unwrap();
                    #impl_fn_name(data);
                }
            }
        });

        quote! {
            #(#tests)*
        }
    };

    quote! {
        /// Original test function
        #input_fn
        #test_functions
    }
}

pub fn test_case_source(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let input_fn: ItemFn = parse2(item).expect("Expected a function");
    let fn_name = input_fn.sig.ident.clone();

    // Parse the source type (enum variant)
    let source: SourceType = parse2(attr).expect("Expected [`rust_test::SourceType`] variant.");

    // Only JsonFile variant exists for now
    let (file_path, type_name): (LitStr, Type) = match source {
        SourceType::JsonFile(path, ty) => (path, ty),
    };

    let file_path_value = file_path.value();

    // Resolve the full path
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path = std::path::Path::new(&manifest_dir).join(&file_path_value);
    let file_path_literal = full_path.to_str().expect("Path contains invalid UTF-8");

    let const_name = format_ident!("{}_DATA", fn_name.to_string().to_uppercase());

    // Read the file
    let file_content = std::fs::read_to_string(&full_path)
        .unwrap_or_else(|_| panic!("Could not read file {}", full_path.display()));

    // Parse JSON and generate tests
    let tests_stream: TokenStream = match serde_json::from_str(&file_content) {
        Ok(Value::Array(array)) => generate_test_set(input_fn, array, fn_name, type_name),
        Ok(single_value) => generate_test_set(input_fn, vec![single_value], fn_name, type_name),
        Err(e) => panic!("Could not parse JSON file {}: {}", file_path_value, e),
    };

    let file_path_const = quote! { const #const_name: &str = include_str!(#file_path_literal); };

    // Output the const + generated tests
    quote! {
        /// --- Test data source
        #file_path_const
        #tests_stream
    }
}
