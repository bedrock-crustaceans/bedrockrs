use de::proto_build_de_struct;
use de::proto_build_de_enum;
use quote::quote;
use ser::proto_build_ser_enum;
use ser::proto_build_ser_struct;
use syn::{parse_macro_input, Data, DeriveInput};


mod de;
mod ser;

#[proc_macro_derive(ProtoCodec, attributes(len_repr, enum_repr))]
pub fn proto_codec_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = match input.data {
        Data::Struct(struct_data) => {
            let ser = proto_build_ser_struct(&struct_data);
            let de = proto_build_de_struct(&struct_data);

            quote! {
                impl #impl_generics bedrockrs_proto_core::ProtoCodec for #name #ty_generics #where_clause {
                    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {
                        #ser
                        Ok(())
                    }

                    fn proto_deserialize(stream: &mut std::io::Cursor<&[u8]>) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {
                        Ok(Self{
                            #de
                        })
                    }
                }
            }
        }
        Data::Enum(enum_data) => {
            let ser = proto_build_ser_enum(&enum_data, &input.attrs, &name);
            let de = proto_build_de_enum(&enum_data, &input.attrs, &name);

            quote! {
                impl #impl_generics bedrockrs_proto_core::ProtoCodec for #name #ty_generics #where_clause {
                    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {
                        #ser
                        Ok(())
                    }

                    fn proto_deserialize(stream: &mut std::io::Cursor<&[u8]>) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {

                        Ok({#de})
                    }
                }
            }
        }
        Data::Union(_) => {
            // Unions are not supported
            panic!(
                "ProtoCodec derive macro only supports named/unnamed structs, got union: {name:?}."
            )
        }
    };

    proc_macro::TokenStream::from(expanded)
}
