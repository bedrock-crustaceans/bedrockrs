use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, Fields, Index};

pub fn proto_build_de_struct(struct_data: &DataStruct) -> TokenStream {
    let fields = &struct_data.fields;

    let expand = match fields {
        Fields::Named(ref fields) => {
            let calls = fields.named.iter().map(|f| {
                let field_name = f.ident.clone();

                quote! {
                    #field_name: match proto_core::ProtoCodec::proto_deserialize(stream) {
                        Ok(v) => { v },
                        Err(e) => { return Err(e) }
                    },
                }
            });

            quote! {
                #(#calls)*
            }
        }
        Fields::Unnamed(ref fields) => {
            let calls = fields.unnamed.iter().enumerate().map(|(i, f)| {
                let index = Index::from(i);

                quote! {
                    #index: match proto_core::ProtoCodec::proto_deserialize(stream) {
                        Ok(v) => { v },
                        Err(e) => { return Err(e) }
                    },
                }
            });

            quote! {
                #(#calls)*
            }
        }
        Fields::Unit => {
            // Unit structs are empty and not supported
            panic!(
                "ProtoCodec macro only supports named/unnamed structs and enums, got unit struct."
            )
        }
    };

    TokenStream::from(expand)
}
