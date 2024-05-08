use bytes::BufMut;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput};

use crate::proto::de::proto_build_de;
use crate::proto::ser::proto_build_ser;

mod proto;

#[proc_macro_derive(MCProtoSerialize)]
pub fn proto_serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;
    let fields = &input.data;

    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ser = proto_build_ser(&input.data);

    let expanded = quote! {
        impl #impl_generics serialize::proto::ser::MCProtoSerialize for #name #ty_generics #where_clause {
            fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), serialize::error::SerilizationError> where Self: Sized {
                #ser
                Ok(())
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(MCProtoDeserialize)]
pub fn proto_deserialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;
    let fields = &input.data;

    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let de = proto_build_de(&input.data);

    let expanded = quote! {
        impl #impl_generics serialize::proto::de::MCProtoDeserialize for #name #ty_generics #where_clause {
            fn proto_deserialize(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, serialize::error::DeserilizationError> where Self: Sized {
                Ok(
                    Self {
                        #de
                    }
                )
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
