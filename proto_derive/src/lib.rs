use de::proto_build_de;
use quote::quote;
use ser::proto_build_ser;
use syn::{DeriveInput, parse_macro_input};

mod de;
mod ser;

#[proc_macro_derive(ProtoCodec)]
pub fn proto_serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;

    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ser = proto_build_ser(&input.data);
    let de = proto_build_de(&input.data);

    let expanded = quote! {
        impl #impl_generics proto_core::ProtoCodec for #name #ty_generics #where_clause {
            fn proto_serialize(&self, stream: &mut bedrock_core::stream::write::ByteStreamWrite) -> Result<(), proto_core::error::ProtoCodecError> where Self: Sized {
                #ser
                Ok(())
            }

            fn proto_deserialize(stream: &mut bedrock_core::stream::read::ByteStreamRead) -> Result<Self, proto_core::error::ProtoCodecError> where Self: Sized {
                Ok(Self{
                    #de
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
