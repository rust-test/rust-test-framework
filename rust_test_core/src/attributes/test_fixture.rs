use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemFn, ItemMod, Item};

pub fn test_fixture(_attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if let Ok(mut input_mod) = parse2::<ItemMod>(item.clone()) {
        process_mod(&mut input_mod)?;
        return Ok(quote!(#input_mod));
    }

    Err(syn::Error::new_spanned(item, "The `#[test_fixture]` attribute can only be applied to a module."))
}

pub fn setup(_attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if let Ok(input_fn) = parse2::<ItemFn>(item.clone()) {
        return Ok(quote!(#input_fn));
    }

    Err(syn::Error::new_spanned(item, "The `#[setup]` attribute can only be applied to a function within a `#[test_fixture]`."))
}

pub fn teardown(_attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if let Ok(input_fn) = parse2::<ItemFn>(item.clone()) {
        return Ok(quote!(#input_fn));
    }

    Err(syn::Error::new_spanned(item, "The `#[teardown]` attribute can only be applied to a function within a `#[test_fixture]`."))
}

fn process_mod(item_mod: &mut ItemMod) -> syn::Result<()> {
    let mod_span = item_mod.ident.span();
    let (_, items) = item_mod.content.as_mut().ok_or_else(|| {
        syn::Error::new(mod_span, "The `#[test_fixture]` attribute can only be applied to an inline module (with `{ ... }`).")
    })?;

    // 1. Find the setup and teardown functions
    let mut setup_fn_name = None;
    let mut teardown_fn_name = None;
    for item in items.iter_mut() {
        if let Item::Fn(item_fn) = item {
            if has_attribute(item_fn, "setup") {
                if setup_fn_name.is_some() {
                    return Err(syn::Error::new_spanned(item_fn, "Only one function can be marked with `#[setup]` in a fixture."));
                }
                setup_fn_name = Some(item_fn.sig.ident.clone());
            }
            if has_attribute(item_fn, "teardown") {
                if teardown_fn_name.is_some() {
                    return Err(syn::Error::new_spanned(item_fn, "Only one function can be marked with `#[teardown]` in a fixture."));
                }
                teardown_fn_name = Some(item_fn.sig.ident.clone());
            }
        }
    }

    if setup_fn_name.is_none() && teardown_fn_name.is_none() {
        return Ok(()); // Nothing to do.
    }

    // 2. Inject calls into tests
    for item in items.iter_mut() {
        if let Item::Fn(item_fn) = item {
            if is_test(item_fn) {
                inject_setup_teardown_calls(item_fn, &setup_fn_name, &teardown_fn_name);
            }
        }
    }

    Ok(())
}

fn has_attribute(item_fn: &ItemFn, attr_name: &str) -> bool {
    item_fn.attrs.iter().any(|attr| {
        attr.path().is_ident(attr_name) || 
        attr.path().segments.last().map(|s| s.ident == attr_name).unwrap_or(false)
    })
}

fn is_test(item_fn: &ItemFn) -> bool {
    item_fn.attrs.iter().any(|attr| {
        attr.path().is_ident("test") || 
        attr.path().segments.last().map(|s| s.ident == "test").unwrap_or(false) ||
        attr.path().is_ident("test_case_source") ||
        attr.path().segments.last().map(|s| s.ident == "test_case_source").unwrap_or(false)
    })
}

fn inject_setup_teardown_calls(item_fn: &mut ItemFn, setup_fn_name: &Option<syn::Ident>, teardown_fn_name: &Option<syn::Ident>) {
    let setup_call = if let Some(setup_fn_name) = setup_fn_name {
        quote! {
            let __setup_result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                #setup_fn_name();
            }));
            if let ::std::prelude::v1::Err(err) = __setup_result {
                let msg = if let Some(s) = err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = err.downcast_ref::<::std::string::String>() {
                    s.clone()
                } else {
                    "Unknown error".to_string()
                };
                panic!("setup failed: {}", msg);
            }
        }
    } else {
        quote!()
    };

    let test_body = item_fn.block.stmts.drain(..);
    let teardown_call = if let Some(teardown_fn_name) = teardown_fn_name {
        quote! {
            let __test_result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                #(#test_body)*
            }));
            
            #teardown_fn_name();

            if let ::std::prelude::v1::Err(err) = __test_result {
                ::std::panic::resume_unwind(err);
            }
        }
    } else {
        quote! {
            #(#test_body)*
        }
    };

    let new_block: syn::Block = parse2(quote!({
        #setup_call
        #teardown_call
    })).expect("Failed to parse setup/teardown call injection");
    
    item_fn.block.stmts = new_block.stmts;
}