use quote::quote;
use rust_test_core::test_params_source;
use std::sync::Mutex;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_test_params_source_errors() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    // Invalid source type
    let attr = quote! { InvalidSource("path") };
    let item = quote! { fn my_test(v: u32) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Expected [`rust_test::SourceType`] variant")
    );

    // No parameters
    let attr = quote! { JsonFile("tests/test_data/test_ddt_data.json") };
    let item = quote! { fn my_test() {} };
    let result = test_params_source(attr, item);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("test generation from source requires at least one parameter")
    );

    // Multiple parameters (now supported)
    let attr = quote! { JsonFile("tests/test_data/test_vec_of_vec.json") };
    let item = quote! { fn my_test(a: Vec<u32>) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());

    // Non-existent file
    let attr = quote! { JsonFile("non_existent.json") };
    let item = quote! { fn my_test(v: u32) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Could not read file")
    );

    // With explicit type in attribute
    let attr = quote! { JsonFile("tests/test_data/test_ddt_data.json", User) };
    let item = quote! { fn my_test(v: User) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());

    // With turbofish
    let attr = quote! { JsonFile::<User>("tests/test_data/test_ddt_data.json") };
    let item = quote! { fn my_test(v: User) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
}

#[test]
fn test_more_errors() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    // Non-typed fn arg (self)
    let item = quote! { fn my_test(self) {} };
    let attr = quote! { JsonFile("tests/test_data/test_ddt_data.json") };
    let result = test_params_source(attr, item);
    // It should now be OK as it's treated as one argument, but it will fail during expansion if type inference fails.
    // However, generate_test_set will try to infer it.
    // Actually, self is a Receiver, not a PatType, so it's ignored in type inference of generate_test_set.
    // If it's the ONLY argument, it might fail to infer type.
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Could not infer type"));

    // Non-Path type (e.g. reference or array)
    let item = quote! { fn my_test(v: [u32; 1]) {} };
    let attr = quote! { JsonFile("tests/test_data/test_ddt_data.json") };
    let result = test_params_source(attr, item);
    assert!(result.is_ok()); // Should still work, just won't be treated as Vec
}

#[test]
fn test_invalid_json() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let attr = quote! { JsonFile("tests/test_data/invalid.json") };
    let item = quote! { fn my_test(v: u32) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Could not parse JSON file")
    );
}

#[test]
fn test_value_variants_coverage() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    // Null
    let attr = quote! { JsonFile("tests/test_data/test_null.json") };
    let item = quote! { fn my_test(v: Option<u32>) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
    assert!(result.unwrap().to_string().contains("my_test__null"));

    // Bool
    let attr = quote! { JsonFile("tests/test_data/test_bool.json") };
    let item = quote! { fn my_test(v: bool) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
    let res_str = result.unwrap().to_string();
    assert!(res_str.contains("my_test__true"));
    assert!(res_str.contains("my_test__false"));

    // Single value (not array)
    let attr = quote! { JsonFile("tests/test_data/test_single_bool.json") };
    let item = quote! { fn my_test(v: bool) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
    let res_str = result.unwrap().to_string();
    assert!(res_str.contains("fn my_test__true"));
}

#[test]
fn test_empty_suffix_variants() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    // Single test case with empty suffix
    let attr = quote! { JsonFile("tests/test_data/test_empty_suffix.json") };
    let item = quote! { fn my_test(v: String) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
    let res_str = result.unwrap().to_string();
    // println!("{}", res_str);
    // test_empty_suffix.json contains [" ", "!!", ""]
    // " " -> suffix "_" -> my_test___
    // "!!" -> suffix "__" -> my_test____
    // "" -> suffix "" -> my_test_2
    assert!(res_str.contains("my_test___"));
    assert!(res_str.contains("my_test____"));
    assert!(res_str.contains("my_test_2"));
}

#[test]
fn test_source_type_parsing_errors() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let item = quote! { fn my_test(v: User) {} };

    // Invalid turbofish arguments (multiple)
    let attr =
        quote! { JsonFile::<User, Another>("tests/test_data/test_ddt_data.json") };
    let result = test_params_source(attr, item.clone());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Expected exactly one type argument")
    );

    // Empty turbofish
    let attr = quote! { JsonFile::<>("tests/test_data/test_ddt_data.json") };
    let result = test_params_source(attr, item.clone());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Expected exactly one type argument")
    );

    // Non-type turbofish argument (const)
    let attr = quote! { JsonFile::<10>("tests/test_data/test_ddt_data.json") };
    let result = test_params_source(attr, item.clone());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Expected a type argument")
    );

    // Unknown variant
    let attr = quote! { UnknownVariant("path") };
    let result = test_params_source(attr, item);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown variant"));
}

#[test]
fn test_is_vec_cases() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    // Case: is_vec = true, all_are_arrays = true
    let attr = quote! { JsonFile("tests/test_data/test_vec_of_vec.json") };
    let item = quote! { fn my_test(v: Vec<u32>) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());

    // Case: is_vec = true, all_are_arrays = false
    let attr = quote! { JsonFile("tests/test_data/test_single_vec.json") };
    let item = quote! { fn my_test(v: Vec<u32>) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
}

#[test]
fn test_comprehensive_coverage() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let attr = quote! { JsonFile("tests/test_data/test_all_variants.json") };
    let item = quote! { fn my_test(v: serde_json::Value) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
    let res_str = result.unwrap().to_string();
    // {"a": 1.2, "b": [true, "a b"]}
    // suffix will be "1_2_true_a_b" (Object values: 1.2 and [true, "a b"])
    // 1.2 -> "1_2"
    // [true, "a b"] -> "true_a_b"
    // Total -> "1_2_true_a_b"
    assert!(res_str.contains("my_test__1_2_true_a_b"));
}

#[test]
fn test_single_empty_suffix() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let attr = quote! { JsonFile("tests/test_data/test_single_empty.json") };
    let item = quote! { fn my_test(v: String) {} };
    let result = test_params_source(attr, item);
    assert!(result.is_ok());
    let res_str = result.unwrap().to_string();
    assert!(res_str.contains("fn my_test ()"));
}

#[test]
fn test_invalid_function_error() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let attr = quote! { JsonFile("tests/test_data/test_ddt_data.json") };
    let item = quote! { struct NotAFunction; };
    let result = test_params_source(attr, item);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Expected a function"));
}

#[test]
fn test_cargo_manifest_dir_not_set() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let attr = quote! { JsonFile("some.json") };
    let item = quote! { fn my_test(v: u32) {} };

    // Save current value
    let original_manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR");

    unsafe {
        std::env::remove_var("CARGO_MANIFEST_DIR");
    }
    let result = test_params_source(attr, item);

    // Restore
    if let Some(val) = original_manifest_dir {
        unsafe {
            std::env::set_var("CARGO_MANIFEST_DIR", val);
        }
    }

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("CARGO_MANIFEST_DIR not set"));
}

#[test]
fn test_invalid_utf8_path() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let attr = quote! { JsonFile("some.json") };
    let item = quote! { fn my_test(v: u32) {} };

    let original_manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR");

    let invalid_utf8 = OsString::from_vec(vec![0xff, 0xfe, 0xfd]);
    unsafe {
        std::env::set_var("CARGO_MANIFEST_DIR", &invalid_utf8);
    }

    let result = test_params_source(attr, item);

    // Restore
    if let Some(val) = original_manifest_dir {
        unsafe {
            std::env::set_var("CARGO_MANIFEST_DIR", &val);
        }
    }

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Path contains invalid UTF-8"));
}