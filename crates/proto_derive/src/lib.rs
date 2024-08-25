use proc_macro::Ident;
use std::collections::HashMap;
use de::proto_build_de_enum;
use de::proto_build_de_struct;
use quote::quote;
use ser::proto_build_ser_enum;
use ser::proto_build_ser_struct;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Token};
use syn::parse::{Parse, ParseStream};

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
                impl #impl_generics ::bedrockrs_proto_core::ProtoCodec for #name #ty_generics #where_clause {
                    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ::bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {
                        #ser
                        Ok(())
                    }

                    fn proto_deserialize(stream: &mut ::std::io::Cursor<&[u8]>) -> Result<Self, ::bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {
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
                impl #impl_generics ::bedrockrs_proto_core::ProtoCodec for #name #ty_generics #where_clause {
                    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ::bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {
                        #ser
                        Ok(())
                    }

                    fn proto_deserialize(stream: &mut ::std::io::Cursor<&[u8]>) -> Result<Self, ::bedrockrs_proto_core::error::ProtoCodecError> where Self: Sized {

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

struct GamepacketInput {
    id: Lit,
    compress: Option<Lit>,
    encrypt: Option<Lit>,
}

impl Parse for GamepacketInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut map = HashMap::new();
        
        loop {
            if !input.peek(syn::Ident) {
                break;
            }

            let param_name = input.parse::<syn::Ident>()?.to_string();
            input.parse::<Token![=]>()?;
            let param_value = input.parse::<syn::Lit>()?;
            
            map.insert(param_name, param_value);

            if !input.peek(Token![,]) {
                break;
            }

            input.parse::<Token![,]>()?;
        }
        
        let id = map.remove(&String::from("id")).unwrap_or_else(|| {
            panic!("Missing id");
        });
        
        Ok(Self {
            id: id.clone(),
            compress: map.remove("compress").and_then(|v| Some(v.clone())),
            encrypt: map.remove("encrypt").and_then(|v| Some(v.clone())),
        })
    }
}

#[proc_macro_attribute]
pub fn gamepacket(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the arguments passed to the attribute
    let args = parse_macro_input!(args as GamepacketInput);
    let item_de = item.clone();
    let derive = parse_macro_input!(item_de as DeriveInput);
    let name = derive.ident;
    
    let id = args.id;
    
    let compress = match args.compress { 
        Some(v) => quote! {#v},
        None => quote! {true},
    };
    
    let encrypt = match args.encrypt {
        Some(v) => quote! {#v},
        None => quote! {true},
    };
    
    let item = proc_macro2::TokenStream::from(item);
    
    let expanded = quote! {
        #item
        
        impl bedrockrs_proto_core::GamePacket for #name {
            const ID: u16 = #id;
            const COMPRESS: bool = #compress;
            const ENCRYPT: bool = #encrypt;
        }
    };
    
    proc_macro::TokenStream::from(expanded)
}

struct GamepacketsInput {
    packets: Vec<(proc_macro2::Ident, proc_macro2::Ident)>
}

impl Parse for GamepacketsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut vec = vec![];

        loop {
            if !input.peek(syn::Ident) {
                break;
            }

            let param_name = input.parse::<syn::Ident>()?;
            input.parse::<Token![=]>()?;
            let param_value = input.parse::<syn::Ident>()?;

            vec.push((param_name, param_value));

            if !input.peek(Token![,]) {
                break;
            }

            input.parse::<Token![,]>()?;
        }
        
        Ok(Self{
            packets: vec,
        })
    }
}

#[proc_macro]
pub fn gamepackets(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as GamepacketsInput);
    
    todo!()
}
