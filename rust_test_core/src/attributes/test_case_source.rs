mod source_type;

pub use crate::attributes::test_case_source::source_type::SourceType;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;
use syn::{parse2, ItemFn, LitStr, Type};

fn serialize_json(value: &Value) -> serde_json::Result<String> {
    #[cfg(test)]
    if std::env::var("FORCE_JSON_ERROR").is_ok() {
        return Err(serde::ser::Error::custom("forced error"));
    }
    serde_json::to_string(value)
}

fn value_to_suffix(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string().replace('.', "_"),
        Value::String(s) => s
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c.to_lowercase().next().unwrap()
                } else {
                    '_'
                }
            })
            .collect(),
        Value::Array(arr) => arr
            .iter()
            .map(value_to_suffix)
            .collect::<Vec<_>>()
            .join("_"),
        Value::Object(obj) => obj
            .values()
            .map(value_to_suffix)
            .collect::<Vec<_>>()
            .join("_"),
    }
}

fn generate_test_set(
    mut input_fn: ItemFn,
    json_array: Vec<Value>,
    fn_name: Ident,
    type_name: Type,
) -> syn::Result<TokenStream> {
    let impl_fn_name = format_ident!("__{}_impl", fn_name);
    input_fn.sig.ident = impl_fn_name.clone();

    let test_functions = if json_array.len() == 1 {
        let value = &json_array[0];
        let json_str = serialize_json(value).map_err(|e| {
            syn::Error::new_spanned(&fn_name, format!("Failed to serialize JSON: {}", e))
        })?;

        let suffix = value_to_suffix(value);
        let test_fn_name = if suffix.is_empty() {
            fn_name.clone()
        } else {
            format_ident!("{}__{}", fn_name, suffix)
        };

        let docstring = format!("Generated test {}", fn_name);

        quote! {
            #[doc = #docstring]
            #[test]
            #[allow(non_snake_case)]
            fn #test_fn_name() {
                let data: #type_name = rust_test_framework::__private::serde_json::from_str(#json_str).unwrap();
                #impl_fn_name(data);
            }
        }
    } else {
        let tests = json_array
            .into_iter()
            .enumerate()
            .map(|(i, value)| {
                let json_str = serialize_json(&value).map_err(|e| {
                    syn::Error::new_spanned(&fn_name, format!("Failed to serialize JSON at index {}: {}", i, e))
                })?;
                
                let suffix = value_to_suffix(&value);
                let test_fn_name = if suffix.is_empty() {
                    format_ident!("{}_{}", fn_name, i)
                } else {
                    format_ident!("{}__{}", fn_name, suffix)
                };

                let docstring = format!("Generated test {} #{}", fn_name, i);

                Ok(quote! {
                    #[doc = #docstring]
                    #[test]
                    #[allow(non_snake_case)]
                    fn #test_fn_name() {
                        let data: #type_name = rust_test_framework::__private::serde_json::from_str(#json_str).unwrap();
                        #impl_fn_name(data);
                    }
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        quote! {
            #(#tests)*
        }
    };

    Ok(quote! {
        /// Original test function
        #input_fn
        #test_functions
    })
}

pub fn test_case_source(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
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
        if input_fn.sig.inputs.len() != 1 {
            return Err(syn::Error::new_spanned(
                &input_fn.sig.inputs,
                "test generation from source supports only 1 type in parameters",
            ));
        }

        if let Some(syn::FnArg::Typed(pat_type)) = input_fn.sig.inputs.first() {
            type_name = Some((*pat_type.ty).clone());
        } else {
            return Err(syn::Error::new_spanned(
                &input_fn.sig.inputs,
                "test generation from source supports only 1 type in parameters",
            ));
        }
    }

    let type_name = type_name.expect("Type should be present by now");

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

    let is_vec = if let Type::Path(type_path) = &type_name {
        type_path.path.segments.last().map(|s| s.ident.to_string() == "Vec").unwrap_or(false)
    } else {
        false
    };

    // Parse JSON and generate tests
    let tests_stream: TokenStream = match serde_json::from_str(&file_content) {
        Ok(Value::Array(array)) => {
            // If expected type is Vec, try parsing it as both list of list and just single list before throwing an error
            if is_vec {
                // Try treating each element as a test case (list of list)
                // We don't really know if it will work until runtime when we deserialize, 
                // but we can try to peek if the first element is also an array.
                // However, the requirement says "try parsing it as both... before throwing an error".
                // Since this is compile time (proc macro), we can't "try parsing" with actual data easily 
                // unless we do it here.
                
                let all_are_arrays = array.iter().all(|v| v.is_array());
                if all_are_arrays {
                    generate_test_set(input_fn, array, fn_name, type_name)?
                } else {
                    // Treat the whole array as a single test case (single list)
                    generate_test_set(input_fn, vec![Value::Array(array)], fn_name, type_name)?
                }
            } else {
                generate_test_set(input_fn, array, fn_name, type_name)?
            }
        }
        Ok(single_value) => generate_test_set(input_fn, vec![single_value], fn_name, type_name)?,
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

#[cfg(test)]
mod internal_tests {
    use super::*;
    use syn::parse_quote;
    use std::sync::Mutex;

    static INTERNAL_ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_value_to_suffix_all_variants() {
        assert_eq!(value_to_suffix(&Value::Null), "null");
        assert_eq!(value_to_suffix(&Value::Bool(true)), "true");
        assert_eq!(value_to_suffix(&Value::Number(serde_json::Number::from(123))), "123");
        assert_eq!(value_to_suffix(&Value::String("A b".to_string())), "a_b");
        assert_eq!(value_to_suffix(&Value::Array(vec![Value::from(1), Value::from(2)])), "1_2");
        let mut map = serde_json::Map::new();
        map.insert("k".to_string(), Value::from(3));
        assert_eq!(value_to_suffix(&Value::Object(map)), "3");
    }

    #[test]
    fn test_generate_test_set_empty_suffix() {
        let _lock = INTERNAL_ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let input_fn: syn::ItemFn = parse_quote! { fn my_test(v: u32) {} };
        let json_array = vec![Value::Null];
        let fn_name = format_ident!("my_test");
        let type_name: syn::Type = parse_quote! { u32 };
        
        let result = generate_test_set(input_fn, json_array, fn_name, type_name);
        assert!(result.is_ok());
        let stream = result.unwrap().to_string();
        assert!(stream.contains("fn my_test__null"));
    }

    #[test]
    fn test_generate_test_set_serialization_error() {
        let _lock = INTERNAL_ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let input_fn: syn::ItemFn = parse_quote! { fn my_test(v: serde_json::Value) {} };
        let fn_name = format_ident!("my_test");
        let type_name: syn::Type = parse_quote! { serde_json::Value };

        // Enable error injection
        unsafe {
            std::env::set_var("FORCE_JSON_ERROR", "true");
        }

        // Case: single value
        let result = generate_test_set(input_fn.clone(), vec![Value::Null], fn_name.clone(), type_name.clone());
        
        let is_err = result.is_err();
        let err_msg = if is_err { result.as_ref().unwrap_err().to_string() } else { String::new() };

        // Case: multiple values
        let result_multi = generate_test_set(input_fn, vec![Value::Null, Value::Null], fn_name, type_name);
        let is_err_multi = result_multi.is_err();
        let err_msg_multi = if is_err_multi { result_multi.as_ref().unwrap_err().to_string() } else { String::new() };

        // Cleanup
        unsafe {
            std::env::remove_var("FORCE_JSON_ERROR");
        }

        assert!(is_err);
        assert!(err_msg.contains("Failed to serialize JSON"));
        
        assert!(is_err_multi);
        assert!(err_msg_multi.contains("Failed to serialize JSON at index 0"));
    }
}
