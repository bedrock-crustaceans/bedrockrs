use proc_macro::TokenStream;
use std::ops::Add;

use html_parser::Node;
use quote::quote;
use syn::{parse_macro_input, ExprTuple, LitStr};

use crate::map::COLOR_CODE_MINECRAFT_MAP;

mod map;

fn parse_node(node: Node, active_color_codes: Vec<&str>) -> String {
    let mut result = String::new();

    match node {
        Node::Text(text) => {
            result.push_str(text.as_str());
        }
        Node::Element(element) => {
            let color_code = COLOR_CODE_MINECRAFT_MAP
                .get(&*element.name)
                .expect(&format!("{} not found in color code map", element.name));

            result.push_str(color_code);

            let mut current_color_codes = active_color_codes.clone();
            current_color_codes.push(color_code);

            for child in element.children {
                result.push_str(&parse_node(child, current_color_codes.clone()));
            }

            if let Some(reset_code) = COLOR_CODE_MINECRAFT_MAP.get("reset") {
                result.push_str(reset_code);
            }

            for code in active_color_codes {
                result.push_str(code);
            }
        }
        Node::Comment(_) => {}
    }

    result
}

#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let dom = match html_parser::Dom::parse(input.value().as_str()) {
        Ok(v) => v,
        Err(e) => {
            panic!("Error while parsing html: {}", e)
        }
    };

    let mut text = String::new();

    for child in dom.children {
        text += parse_node(child, vec![]).as_str();
    }

    TokenStream::from(quote! {
        #text
    })
}

#[proc_macro]
pub fn colorf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprTuple);

    // let dom = match html_parser::Dom::parse(input.value().as_str()) {
    //     Ok(v) => v,
    //     Err(e) => {
    //         panic!("Error while parsing html: {}", e)
    //     }
    // };

    let mut text = String::new();

    // for child in dom.children {
    //     text += parse_node(child, vec![]).as_str();
    // }

    TokenStream::from(quote! {
        #text
    })
}
