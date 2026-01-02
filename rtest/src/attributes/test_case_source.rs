use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::parse::Parse;
use syn::{parse_macro_input, ItemFn, LitStr, Token, Type};

struct TestCaseArgs {
    file_path: LitStr,
    type_name: Type,
}

impl Parse for TestCaseArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file_path: LitStr = input.parse()?; // first argument: string literal
        input.parse::<Token![,]>()?; // comma
        let type_name: Type = input.parse()?; // second argument: type
        Ok(TestCaseArgs {
            file_path,
            type_name,
        })
    }
}

pub fn test_case_source(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.sig.ident.clone();

    // Parse macro attribute arguments
    let args = parse_macro_input!(attr as TestCaseArgs);
    let file_path = args.file_path.value();
    let type_name = args.type_name;

    // Resolve JSON file path relative to consuming crate
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path = std::path::Path::new(&manifest_dir).join(&file_path);

    let file_content = std::fs::read_to_string(&full_path)
        .unwrap_or_else(|_| panic!("Could not read file {}", full_path.display()));

    // Parse JSON array
    let json_array: Vec<Value> = serde_json::from_str(&file_content)
        .expect("Expected a JSON array in the test data file");

    // Generate one #[test] function per array element
    let test_functions = json_array.into_iter().enumerate().map(|(i, value)| {
        let json_str = serde_json::to_string(&value).unwrap();
        let test_fn_name = format_ident!("{}_case_{}", fn_name, i);

        quote! {
                #[test]
                fn #test_fn_name() {
                    let data: #type_name = serde_json::from_str(#json_str).unwrap();
                    #fn_name(data);
                }
            }
    });

    // Return the original function plus all generated test functions
    let expanded = quote! {
            #input_fn
            #(#test_functions)*
        };

    TokenStream::from(expanded)
}
