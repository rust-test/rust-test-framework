use crate::attributes::common::generate_test_set;
use proc_macro2::TokenStream;
use serde_json::Value;
use syn::parse::{Parse, ParseStream};
use syn::{parse2, Expr, ItemFn, Lit, Token};

pub fn test_params(_attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let input_fn: ItemFn =
        parse2(item).map_err(|e| syn::Error::new(e.span(), format!("Expected a function: {}", e)))?;
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

fn expr_to_value(expr: &Expr) -> syn::Result<Value> {
    match expr {
        Expr::Lit(expr_lit) => lit_to_value(&expr_lit.lit),
        Expr::Path(expr_path) => {
            // Treat path as enum unit variant
            if let Some(segment) = expr_path.path.segments.last() {
                let name = segment.ident.to_string();
                if name == "None" && expr_path.path.segments.len() == 1 {
                    Ok(Value::Null)
                } else {
                    Ok(Value::String(name))
                }
            } else {
                Err(syn::Error::new_spanned(expr, "Invalid path"))
            }
        }
        Expr::Call(expr_call) => {
            let (variant_name, segments_len) = if let Expr::Path(expr_path) = &*expr_call.func {
                let name = expr_path
                    .path
                    .segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .ok_or_else(|| syn::Error::new_spanned(&expr_call.func, "Invalid variant name"))?;
                (name, expr_path.path.segments.len())
            } else {
                return Err(syn::Error::new_spanned(
                    &expr_call.func,
                    "Expected a variant name",
                ));
            };

            let mut args = Vec::new();
            for arg in &expr_call.args {
                args.push(expr_to_value(arg)?);
            }

            if variant_name == "Some" && segments_len == 1 && args.len() == 1 {
                return Ok(args[0].clone());
            }

            if segments_len > 1 {
                // Treat as enum variant (externally tagged)
                let mut map = serde_json::Map::new();
                if args.len() == 1 {
                    map.insert(variant_name, args[0].clone());
                } else {
                    map.insert(variant_name, Value::Array(args));
                }
                Ok(Value::Object(map))
            } else {
                // Treat as tuple struct or special variant (like Some)
                // If it's a single segment, we still might want it tagged for things like Some(1) -> {"Some": 1}
                // But if it's Point(1, 2) -> [1, 2]
                // This is ambiguous. 
                // However, for Option/Result, they are usually in prelude, so they have 1 segment.
                // If we want Point(1, 2) to work as a struct, it should be [1, 2].
                // Let's stick to: if it's 1 segment, we wrap it ONLY if it's a known common enum like Some/Ok/Err? 
                // No, let's keep it simple: for ExprCall, if it's 1 segment, maybe we should STILL wrap it?
                // Because tuple structs are less common to be initialized this way in tests compared to enums.
                // Actually, if I have Point(1, 2) it's likely a tuple struct.
                
                // Let's see what the user asked: "add support for explicit initialization of struct, rust-style (Point {x: 1, y: 2)"
                // They specifically mentioned the struct initialization style with braces.
                
                // For ExprCall (parentheses), I'll keep the current behavior for now or only change it if requested.
                // Actually, to be consistent, if it's 1 segment, it should probably NOT be wrapped if we want it to be a tuple struct.
                // But Some(1) would then be [1] instead of {"Some": 1}. That would break Option.
                
                // So I'll only change ExprStruct for now.
                let mut map = serde_json::Map::new();
                if args.len() == 1 {
                    map.insert(variant_name, args[0].clone());
                } else {
                    map.insert(variant_name, Value::Array(args));
                }
                Ok(Value::Object(map))
            }
        }
        Expr::Struct(expr_struct) => {
            let mut fields = serde_json::Map::new();
            for field in &expr_struct.fields {
                let field_name = if let syn::Member::Named(ident) = &field.member {
                    ident.to_string()
                } else {
                    return Err(syn::Error::new_spanned(&field.member, "Expected named field"));
                };
                fields.insert(field_name, expr_to_value(&field.expr)?);
            }

            if expr_struct.path.segments.len() > 1 {
                let variant_name = expr_struct
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string();
                let mut map = serde_json::Map::new();
                map.insert(variant_name, Value::Object(fields));
                Ok(Value::Object(map))
            } else {
                // Direct struct initialization
                Ok(Value::Object(fields))
            }
        }
        Expr::Unary(expr_unary) => {
            if let syn::UnOp::Neg(_) = expr_unary.op {
                let val = expr_to_value(&expr_unary.expr)?;
                match val {
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            Ok(Value::Number((-i).into()))
                        } else if let Some(f) = n.as_f64() {
                            if let Some(neg_f) = serde_json::Number::from_f64(-f) {
                                Ok(Value::Number(neg_f))
                            } else {
                                Err(syn::Error::new_spanned(expr, "Invalid float after negation"))
                            }
                        } else {
                            Err(syn::Error::new_spanned(expr, "Unsupported number for negation"))
                        }
                    }
                    _ => Err(syn::Error::new_spanned(expr, "Negation only supported for numbers")),
                }
            } else {
                Err(syn::Error::new_spanned(expr, "Unsupported unary operator"))
            }
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "Unsupported expression type for test_params. Use literals or enum variants.",
        )),
    }
}

fn lit_to_value(lit: &Lit) -> syn::Result<Value> {
    match lit {
        Lit::Str(s) => Ok(Value::String(s.value())),
        Lit::Int(i) => {
            let n = i.base10_parse::<i64>().map_err(|e| syn::Error::new(i.span(), e))?;
            Ok(Value::Number(n.into()))
        }
        Lit::Float(f) => {
            let n = f.base10_parse::<f64>().map_err(|e| syn::Error::new(f.span(), e))?;
            let n = serde_json::Number::from_f64(n)
                .ok_or_else(|| syn::Error::new(f.span(), "Invalid float"))?;
            Ok(Value::Number(n))
        }
        Lit::Bool(b) => Ok(Value::Bool(b.value)),
        _ => Err(syn::Error::new_spanned(
            lit,
            "Unsupported literal type for test_params",
        )),
    }
}
