use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;
use syn::{ItemFn, Type};

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
    json_array: Vec<Value>,
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
        let value = &json_array[0];
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
            .map(|(i, value)| {
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
