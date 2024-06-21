use proc_macro::TokenStream;
use std::fmt::format;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let dom = match html_parser::Dom::parse(input.value().as_str()) {
        Ok(v) => { v }
        Err(e) => { panic!(e) }
    };

    let text = format!("{dom:#?}");

    TokenStream::from(quote!{
        #text
    })
}
