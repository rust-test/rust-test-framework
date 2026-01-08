use crate::attributes::common::{generate_test_set, ValueWithSpan};
use proc_macro2::Span;
use quote::format_ident;
use serde_json::Value;
use std::sync::Mutex;
use syn::parse_quote;

static INTERNAL_ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_generate_test_set_empty_suffix() {
    let _lock = INTERNAL_ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let input_fn: syn::ItemFn = parse_quote! { fn my_test(v: u32) {} };
    let json_array = vec![ValueWithSpan {
        value: Value::Null,
        span: Span::call_site(),
        suffix: None,
    }];
    let fn_name = format_ident!("my_test");
    let type_name: syn::Type = parse_quote! { u32 };

    let result = generate_test_set(input_fn, json_array, fn_name, Some(type_name));
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
    let result = generate_test_set(
        input_fn.clone(),
        vec![ValueWithSpan {
            value: Value::Null,
            span: Span::call_site(),
            suffix: None,
        }],
        fn_name.clone(),
        Some(type_name.clone()),
    );

    let is_err = result.is_err();
    let err_msg = if is_err {
        result.as_ref().unwrap_err().to_string()
    } else {
        String::new()
    };

    // Case: multiple values
    let result_multi = generate_test_set(
        input_fn,
        vec![
            ValueWithSpan {
                value: Value::Null,
                span: Span::call_site(),
                suffix: None,
            },
            ValueWithSpan {
                value: Value::Null,
                span: Span::call_site(),
                suffix: None,
            },
        ],
        fn_name,
        Some(type_name),
    );
    let is_err_multi = result_multi.is_err();
    let err_msg_multi = if is_err_multi {
        result_multi.as_ref().unwrap_err().to_string()
    } else {
        String::new()
    };

    // Cleanup
    unsafe {
        std::env::remove_var("FORCE_JSON_ERROR");
    }

    assert!(is_err);
    assert!(err_msg.contains("Failed to serialize JSON"));

    assert!(is_err_multi);
    assert!(err_msg_multi.contains("Failed to serialize JSON at index 0"));
}