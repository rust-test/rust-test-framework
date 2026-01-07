use crate::attributes::common::{expr_to_value, generate_test_set, parse_item_fn};
use proc_macro2::TokenStream;
use serde_json::Value;
use syn::parse::{Parse, ParseStream};
use syn::{parse2, Expr, Token};

pub fn test_params(_attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let input_fn = parse_item_fn(item)?;
    let fn_name = input_fn.sig.ident.clone();

    let args: TestCaseArgs = parse2(_attr)?;
    let arg_count = input_fn.sig.inputs.len();

    let values = if arg_count > 1 {
        if args.values.len() != arg_count {
            return Err(syn::Error::new_spanned(
                &input_fn.sig.inputs,
                format!(
                    "Test function expects {} arguments, but {} were provided in #[test_params]",
                    arg_count,
                    args.values.len()
                ),
            ));
        }
        vec![Value::Array(args.values)]
    } else if arg_count == 1 {
        if args.values.len() > 1 {
            vec![Value::Array(args.values)]
        } else {
            vec![args.values[0].clone()]
        }
    } else {
        return Err(syn::Error::new_spanned(
            &input_fn.sig.inputs,
            "Test function must have at least one argument when using #[test_params]",
        ));
    };

    generate_test_set(input_fn, values, fn_name, None)
}

struct TestCaseArgs {
    values: Vec<Value>,
}

impl Parse for TestCaseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut values = Vec::new();
        while !input.is_empty() {
            if input.peek(syn::token::Paren) {
                let content;
                syn::parenthesized!(content in input);
                let mut tuple_values = Vec::new();
                while !content.is_empty() {
                    let expr: Expr = content.parse()?;
                    tuple_values.push(expr_to_value(&expr)?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    } else {
                        break;
                    }
                }
                values.push(Value::Array(tuple_values));
            } else {
                let expr: Expr = input.parse()?;
                values.push(expr_to_value(&expr)?);
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        if values.is_empty() {
            return Err(input.error("Expected at least one test case value"));
        }

        Ok(TestCaseArgs { values })
    }
}
