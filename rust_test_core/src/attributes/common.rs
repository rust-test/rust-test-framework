use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;
use syn::{Expr, ItemFn, Lit, Type};

fn is_path_type(ty: &Type) -> bool {
    let check_path = |path: &syn::Path| {
        path.segments.last().map_or(false, |s| {
            let name = s.ident.to_string();
            name == "Path" || name == "PathBuf"
        })
    };

    match ty {
        Type::Path(type_path) => check_path(&type_path.path),
        Type::Reference(type_ref) => {
            if let Type::Path(type_path) = &*type_ref.elem {
                check_path(&type_path.path)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn resolve_path(path_str: &str) -> Option<PathBuf> {
    let path = Path::new(path_str);
    if path.exists() {
        return Some(path.to_path_buf());
    }

    // Try relative to CARGO_MANIFEST_DIR if it's set
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let full_path = Path::new(&manifest_dir).join(path);
        if full_path.exists() {
            return Some(full_path);
        }
    }

    None
}

#[derive(Clone)]
pub struct ValueWithSpan {
    pub value: Value,
    pub span: Span,
}

pub fn expr_to_value_with_span(expr: &Expr) -> syn::Result<ValueWithSpan> {
    let value = expr_to_value(expr)?;
    Ok(ValueWithSpan {
        value,
        span: expr.span(),
    })
}

pub fn expr_to_value(expr: &Expr) -> syn::Result<Value> {
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

            Ok(to_tagged_object(variant_name, args))
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
                Ok(to_tagged_object(variant_name, vec![Value::Object(fields)]))
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
            "Unsupported expression type. Use literals, enum variants or struct initializers.",
        )),
    }
}

fn to_tagged_object(variant_name: String, args: Vec<Value>) -> Value {
    let mut map = serde_json::Map::new();
    if args.len() == 1 {
        map.insert(variant_name, args[0].clone());
    } else {
        map.insert(variant_name, Value::Array(args));
    }
    Value::Object(map)
}

pub fn lit_to_value(lit: &Lit) -> syn::Result<Value> {
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
            "Unsupported literal type",
        )),
    }
}

pub fn parse_item_fn(item: TokenStream) -> syn::Result<ItemFn> {
    syn::parse2(item).map_err(|e| syn::Error::new(e.span(), format!("Expected a function: {}", e)))
}

pub fn serialize_json(value: &Value) -> serde_json::Result<String> {
    #[cfg(test)]
    if std::env::var("FORCE_JSON_ERROR").is_ok() {
        return Err(serde::ser::Error::custom("forced error"));
    }
    serde_json::to_string(value)
}

pub fn value_to_suffix(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string().replace('.', "_").replace('-', "_"),
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

pub fn generate_test_set(
    mut input_fn: ItemFn,
    json_array: Vec<ValueWithSpan>,
    fn_name: Ident,
    type_name: Option<Type>,
) -> syn::Result<TokenStream> {
    let fn_name_str = fn_name.to_string();
    let (real_fn_name, impl_fn_name) = if fn_name_str.starts_with("__") && fn_name_str.ends_with("_impl") && fn_name_str.len() > 7 {
        let base = &fn_name_str[2..fn_name_str.len() - 5];
        (format_ident!("{}", base), fn_name.clone())
    } else {
        (fn_name.clone(), format_ident!("__{}_impl", fn_name))
    };

    input_fn.sig.ident = impl_fn_name.clone();

    let is_tuple = input_fn.sig.inputs.len() > 1;
    let type_token = if let Some(tn) = type_name {
        quote!(#tn)
    } else if is_tuple {
        let types = input_fn.sig.inputs.iter().map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                let ty = &pat_type.ty;
                quote!(#ty)
            } else {
                quote!(())
            }
        });
        quote!((#(#types),*))
    } else if input_fn.sig.inputs.len() == 1 {
        if let Some(syn::FnArg::Typed(pat_type)) = input_fn.sig.inputs.first() {
            let ty = &pat_type.ty;
            quote!(#ty)
        } else {
            return Err(syn::Error::new_spanned(
                &input_fn.sig.inputs,
                "Could not infer type for test case. Please provide it explicitly.",
            ));
        }
    } else {
        return Err(syn::Error::new_spanned(
            &input_fn.sig.inputs,
            "Could not infer type for test case. Please provide it explicitly.",
        ));
    };

    let test_functions = if json_array.len() == 1 {
        let value_with_span = &json_array[0];
        let value = &value_with_span.value;

        // Check for Path existence if applicable
        if input_fn.sig.inputs.len() == 1 {
            if let Some(syn::FnArg::Typed(pat_type)) = input_fn.sig.inputs.first() {
                if is_path_type(&pat_type.ty) {
                    if let Value::String(path_str) = value {
                        if resolve_path(path_str).is_none() {
                            return Err(syn::Error::new(
                                value_with_span.span,
                                format!("Path not found: {}", path_str),
                            ));
                        }
                    }
                }
            }
        }

        generate_single_test(
            &real_fn_name,
            &impl_fn_name,
            value,
            None,
            &type_token,
            is_tuple,
            input_fn.sig.inputs.len(),
        )?
    } else {
        let tests = json_array
            .into_iter()
            .enumerate()
            .map(|(i, value_with_span)| {
                let value = &value_with_span.value;
                // Check for Path existence if applicable
                if input_fn.sig.inputs.len() == 1 {
                    if let Some(syn::FnArg::Typed(pat_type)) = input_fn.sig.inputs.first() {
                        if is_path_type(&pat_type.ty) {
                            if let Value::String(path_str) = &value {
                                if resolve_path(path_str).is_none() {
                                    return Err(syn::Error::new(
                                        value_with_span.span,
                                        format!("Path not found: {}", path_str),
                                    ));
                                }
                            }
                        }
                    }
                }

                generate_single_test(
                    &real_fn_name,
                    &impl_fn_name,
                    &value,
                    Some(i),
                    &type_token,
                    is_tuple,
                    input_fn.sig.inputs.len(),
                )
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

fn generate_single_test(
    fn_name: &Ident,
    impl_fn_name: &Ident,
    value: &Value,
    index: Option<usize>,
    type_token: &TokenStream,
    is_tuple: bool,
    arg_count: usize,
) -> syn::Result<TokenStream> {
    let json_str = serialize_json(value).map_err(|e| {
        let msg = if let Some(i) = index {
            format!("Failed to serialize JSON at index {}: {}", i, e)
        } else {
            format!("Failed to serialize JSON: {}", e)
        };
        syn::Error::new_spanned(fn_name, msg)
    })?;

    let suffix = value_to_suffix(value);
    let test_fn_name = if suffix.is_empty() {
        if let Some(i) = index {
            format_ident!("{}_{}", fn_name, i)
        } else {
            fn_name.clone()
        }
    } else {
        format_ident!("{}__{}", fn_name, suffix)
    };

    let docstring = if let Some(i) = index {
        format!("Generated test {} #{}", fn_name, i)
    } else {
        format!("Generated test {}", fn_name)
    };

    let call_expr = if is_tuple {
        let idents: Vec<_> = (0..arg_count)
            .map(|i| format_ident!("arg_{}", i))
            .collect();
        quote! {
            let (#(#idents),*): #type_token = rust_test_framework::__private::serde_json::from_str(#json_str).unwrap();
            #impl_fn_name(#(#idents),*);
        }
    } else {
        quote! {
            let data: #type_token = rust_test_framework::__private::serde_json::from_str(#json_str).unwrap();
            #impl_fn_name(data);
        }
    };

    Ok(quote! {
        #[doc = #docstring]
        #[test]
        #[allow(non_snake_case)]
        fn #test_fn_name() {
            #call_expr
        }
    })
}
