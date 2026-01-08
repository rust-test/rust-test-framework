mod source_type;

pub use crate::attributes::test_params_source::source_type::SourceType;
use crate::attributes::common::{generate_test_set, parse_item_fn, ValueWithSpan, is_path_type};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::{parse2, LitStr, Type};
use std::sync::LazyLock;

static CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(|| {
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("Failed to create reqwest client")
});

pub fn test_params_source(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let source: SourceType = parse2(attr).map_err(|e| {
        syn::Error::new(
            e.span(),
            format!("Expected [`rust_test::SourceType`] variant: {}", e),
        )
    })?;
    let source_span = source.span();
    let input_fn = parse_item_fn(item)?;
    let fn_name = input_fn.sig.ident.clone();

    // 1. Extract parameter type from function if not provided in attribute
    let (json_content, mut type_name, file_info): (String, Option<Type>, Option<(LitStr, String)>) = match source {
        SourceType::JsonFile(ref path, ref ty, _) => {
            let file_path_value = path.value();
            // Resolve the full path
            let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| {
                syn::Error::new_spanned(path, "CARGO_MANIFEST_DIR not set")
            })?;
            let full_path = std::path::Path::new(&manifest_dir).join(&file_path_value);
            let file_path_literal = full_path.to_str().ok_or_else(|| {
                syn::Error::new_spanned(path, "Path contains invalid UTF-8")
            })?;

            // Read the file
            let content = std::fs::read_to_string(&full_path).map_err(|e| {
                syn::Error::new_spanned(
                    path,
                    format!("Could not read file {}: {}", full_path.display(), e),
                )
            })?;
            (content, ty.clone(), Some((path.clone(), file_path_literal.to_string())))
        }
        SourceType::JsonString(ref json_str, ref ty, _) => (json_str.value(), ty.clone(), None),
        SourceType::JsonResponse(ref url, ref ty, _) => {
            let url_value = url.value();
            let response = CLIENT.get(&url_value)
                .send()
                .map_err(|e| {
                    syn::Error::new_spanned(
                        url,
                        format!("Could not fetch URL {}: {}", url_value, e),
                    )
                })?;
            
            if !response.status().is_success() {
                return Err(syn::Error::new_spanned(
                    url,
                    format!("Could not fetch URL {}: status code {}", url_value, response.status()),
                ));
            }

            let content = response.text()
                .map_err(|e| {
                    syn::Error::new_spanned(
                        url,
                        format!("Could not read response from {}: {}", url_value, e),
                    )
                })?;
            (content, ty.clone(), None)
        }
        SourceType::PathMask(ref mask, _) => {
            let mask_value = mask.value();
            let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| {
                syn::Error::new_spanned(mask, "CARGO_MANIFEST_DIR not set")
            })?;
            let full_mask = std::path::Path::new(&manifest_dir).join(&mask_value);
            let full_mask_str = full_mask.to_str().ok_or_else(|| {
                syn::Error::new_spanned(mask, "Path mask contains invalid UTF-8")
            })?;

            let matches: Vec<_> = glob::glob(full_mask_str)
                .map_err(|e| syn::Error::new_spanned(mask, format!("Invalid glob pattern: {}", e)))?
                .filter_map(Result::ok)
                .collect();

            if matches.is_empty() {
                return Err(syn::Error::new_spanned(
                    mask,
                    format!("No files matched pattern: {}", mask_value),
                ));
            }

            let paths: Vec<Value> = matches
                .into_iter()
                .filter_map(|p| {
                    p.strip_prefix(&manifest_dir)
                        .ok()
                        .and_then(|p| p.to_str())
                        .map(|s| Value::String(s.to_string()))
                })
                .collect();

            (serde_json::to_string(&paths).unwrap(), None, None)
        }
    };

    if let SourceType::PathMask(_, _) = source {
        if input_fn.sig.inputs.len() != 1 {
            return Err(syn::Error::new_spanned(
                &input_fn.sig.inputs,
                "PathMask requires exactly one parameter",
            ));
        }
        if let Some(syn::FnArg::Typed(pat_type)) = input_fn.sig.inputs.first() {
            if !is_path_type(&pat_type.ty) {
                return Err(syn::Error::new_spanned(
                    &pat_type.ty,
                    "PathMask requires a &Path or PathBuf parameter",
                ));
            }
        }
    }

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

    let is_vec = if let Some(Type::Path(type_path)) = &type_name {
        type_path.path.segments.last().map(|s| s.ident.to_string() == "Vec").unwrap_or(false)
    } else {
        false
    };

    let type_name_opt = type_name;

    // Parse JSON and generate tests
    let tests_stream: TokenStream = match serde_json::from_str(&json_content) {
        Ok(Value::Array(array)) => {
            // If expected type is Vec, try parsing it as both list of list and just single list before throwing an error
            if is_vec {
                let all_are_arrays = array.iter().all(|v| v.is_array());
                if all_are_arrays {
                    generate_test_set(
                        input_fn,
                        array
                            .into_iter()
                            .map(|v| ValueWithSpan {
                                value: v,
                                span: source_span,
                                suffix: None,
                            })
                            .collect(),
                        fn_name.clone(),
                        type_name_opt,
                    )?
                } else {
                    // Treat the whole array as a single test case (single list)
                    generate_test_set(
                        input_fn,
                        vec![ValueWithSpan {
                            value: Value::Array(array),
                            span: source_span,
                            suffix: None,
                        }],
                        fn_name.clone(),
                        type_name_opt,
                    )?
                }
            } else {
                generate_test_set(
                    input_fn,
                    array
                        .into_iter()
                        .map(|v| ValueWithSpan {
                            value: v,
                            span: source_span,
                            suffix: None,
                        })
                        .collect(),
                    fn_name.clone(),
                    type_name_opt,
                )?
            }
        }
        Ok(single_value) => {
            if is_vec {
                return Err(syn::Error::new(
                    source_span,
                    format!("Expected JSON array for Vec type, but got: {}", single_value),
                ));
            }
            generate_test_set(
                input_fn,
                vec![ValueWithSpan {
                    value: single_value,
                    span: source_span,
                    suffix: None,
                }],
                fn_name.clone(),
                type_name_opt,
            )?
        }
        Err(e) => {
            if let Some((file_path, _)) = &file_info {
                return Err(syn::Error::new_spanned(
                    file_path,
                    format!("Could not parse JSON file {}: {}", file_path.value(), e),
                ));
            } else {
                return Err(syn::Error::new(
                    source_span,
                    format!("Could not parse JSON: {}", e),
                ));
            }
        }
    };

    if let Some((_file_path, file_path_literal)) = file_info {
        let const_name = format_ident!("{}_DATA", fn_name.to_string().to_uppercase());
        let file_path_const = quote! { const #const_name: &str = include_str!(#file_path_literal); };
        // Output the const + generated tests
        Ok(quote! {
            /// --- Test data source
            #file_path_const
            #tests_stream
        })
    } else {
        Ok(tests_stream)
    }
}
