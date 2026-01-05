mod attributes;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test_case_source(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_case_source(attr.into(), item.into()).into()
}
