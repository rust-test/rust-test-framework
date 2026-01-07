#[cfg(test)]
mod tests {
    use quote::quote;
    use rust_test_core::test_case_source;

    #[test]
    fn test_test_case_source_errors() {
        // Invalid source type
        let attr = quote! { InvalidSource("path") };
        let item = quote! { fn my_test(v: u32) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected [`rust_test::SourceType`] variant")
        );

        // No parameters
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_ddt_data.json") };
        let item = quote! { fn my_test() {} };
        let result = test_case_source(attr, item);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("test generation from source supports only 1 type in parameters")
        );

        // Too many parameters
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_ddt_data.json") };
        let item = quote! { fn my_test(a: u32, b: u32) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("test generation from source supports only 1 type in parameters")
        );

        // Non-existent file
        let attr = quote! { JsonFile("non_existent.json") };
        let item = quote! { fn my_test(v: u32) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Could not read file")
        );

        // With explicit type in attribute
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_ddt_data.json", User) };
        let item = quote! { fn my_test(v: User) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());

        // With turbofish
        let attr = quote! { JsonFile::<User>("../rust_test_framework/tests/test_ddt_data.json") };
        let item = quote! { fn my_test(v: User) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());
    }

    #[test]
    fn test_more_errors() {
        // Non-typed fn arg (self)
        let item = quote! { fn my_test(self) {} };
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_ddt_data.json") };
        let result = test_case_source(attr, item);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("test generation from source supports only 1 type in parameters")
        );

        // Non-Path type (e.g. reference or array)
        let item = quote! { fn my_test(v: [u32; 1]) {} };
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_ddt_data.json") };
        let result = test_case_source(attr, item);
        assert!(result.is_ok()); // Should still work, just won't be treated as Vec
    }

    #[test]
    fn test_invalid_json() {
        let attr = quote! { JsonFile("tests/test_data/invalid.json") };
        let item = quote! { fn my_test(v: u32) {} };
        let result = test_case_source(attr, item);
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
        // Null
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_null.json") };
        let item = quote! { fn my_test(v: Option<u32>) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());
        assert!(result.unwrap().to_string().contains("my_test__null"));

        // Bool
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_bool.json") };
        let item = quote! { fn my_test(v: bool) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());
        let res_str = result.unwrap().to_string();
        assert!(res_str.contains("my_test__true"));
        assert!(res_str.contains("my_test__false"));

        // Single value (not array)
        let attr = quote! { JsonFile("tests/test_data/test_single_bool.json") };
        let item = quote! { fn my_test(v: bool) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());
        let res_str = result.unwrap().to_string();
        assert!(res_str.contains("fn my_test__true"));
    }

    #[test]
    fn test_empty_suffix_variants() {
        // Single test case with empty suffix
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_empty_suffix.json") };
        let item = quote! { fn my_test(v: String) {} };
        let result = test_case_source(attr, item);
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
        let item = quote! { fn my_test(v: User) {} };

        // Invalid turbofish arguments (multiple)
        let attr =
            quote! { JsonFile::<User, Another>("../rust_test_framework/tests/test_ddt_data.json") };
        let result = test_case_source(attr, item.clone());
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected exactly one type argument")
        );

        // Empty turbofish
        let attr = quote! { JsonFile::<>("../rust_test_framework/tests/test_ddt_data.json") };
        let result = test_case_source(attr, item.clone());
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected exactly one type argument")
        );

        // Non-type turbofish argument (const)
        let attr = quote! { JsonFile::<10>("../rust_test_framework/tests/test_ddt_data.json") };
        let result = test_case_source(attr, item.clone());
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected a type argument")
        );

        // Unknown variant
        let attr = quote! { UnknownVariant("path") };
        let result = test_case_source(attr, item);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown variant"));
    }

    #[test]
    fn test_is_vec_cases() {
        // Case: is_vec = true, all_are_arrays = true
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_vec_of_vec.json") };
        let item = quote! { fn my_test(v: Vec<u32>) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());

        // Case: is_vec = true, all_are_arrays = false
        let attr = quote! { JsonFile("../rust_test_framework/tests/test_single_vec.json") };
        let item = quote! { fn my_test(v: Vec<u32>) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());
    }

    #[test]
    fn test_comprehensive_coverage() {
        let attr = quote! { JsonFile("tests/test_data/test_all_variants.json") };
        let item = quote! { fn my_test(v: serde_json::Value) {} };
        let result = test_case_source(attr, item);
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
        let attr = quote! { JsonFile("tests/test_data/test_single_empty.json") };
        let item = quote! { fn my_test(v: String) {} };
        let result = test_case_source(attr, item);
        assert!(result.is_ok());
        let res_str = result.unwrap().to_string();
        assert!(res_str.contains("fn my_test ()"));
    }
}
