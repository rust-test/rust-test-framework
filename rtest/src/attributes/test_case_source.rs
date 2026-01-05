use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::parse::Parse;
use syn::{parse2, ItemFn, LitStr, Token, Type};

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
    let input_fn: ItemFn = parse2(item).expect("Expected a function");
    let fn_name = input_fn.sig.ident.clone();

    let args: TestCaseArgs = parse2(attr).expect("Expected test case source attributes");
    let file_path = args.file_path.value();
    let type_name = args.type_name;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path = std::path::Path::new(&manifest_dir).join(&file_path);
    let file_path_literal = full_path.to_str().unwrap();
    let generated = quote! {
        const JSON_DATA: &str = include_str!(#file_path_literal);
    };

    let file_content = std::fs::read_to_string(&full_path)
        .unwrap_or_else(|_| panic!("Could not read file {}", full_path.display()));

    let json_array: Vec<Value> = serde_json::from_str(&file_content)
        .expect("Expected a JSON array in the test data file");

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

    let expanded = quote! {
            #input_fn

            #generated

            #(#test_functions)*
        };

    TokenStream::from(expanded)
}
