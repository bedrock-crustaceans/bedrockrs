use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Fields, Index};

pub fn proto_build_ser_struct(struct_data: &DataStruct) -> TokenStream {
    let fields = &struct_data.fields;

    let expand = match fields {
        Fields::Named(ref fields) => {
            let calls = fields.named.iter().map(|f| {
                let field_name = f.ident.clone();

                quote! {
                    match proto_core::ProtoCodec::proto_serialize(&self.#field_name, stream) {
                        Ok(_) => { },
                        Err(e) => { return Err(e) }
                    };
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
                    match proto_core::ProtoCodec::proto_serialize(&self.#index, stream) {
                        Ok(_) => { },
                        Err(e) => { return Err(e) }
                    };
                }
            });

            quote! {
                #(#calls)*
            }
        }
        Fields::Unit => {
            // Unit structs are empty and not supported
            panic!("ProtoCodec macro only supports named/unnamed structs and enums, got unit struct.")
        }
    };

    TokenStream::from(expand)
}
