use de::proto_build_de_enum;
use de::proto_build_de_struct;
use proc_macro::Ident;
use quote::quote;
use ser::proto_build_ser_enum;
use ser::proto_build_ser_struct;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Data, DeriveInput, Lit, Token};

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
pub fn gamepacket(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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

        impl ::bedrockrs_proto_core::GamePacket for #name {
            const ID: u16 = #id;
            const COMPRESS: bool = #compress;
            const ENCRYPT: bool = #encrypt;
        }
    };

    proc_macro::TokenStream::from(expanded)
}

struct GamepacketsInput {
    packets: Vec<(proc_macro2::Ident, Option<proc_macro2::Ident>)>,
}

impl Parse for GamepacketsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut vec = vec![];

        loop {
            if !input.peek(syn::Ident) {
                break;
            }

            let param_name = input.parse::<syn::Ident>()?;
            input.parse::<Token![:]>()?;

            let param_value = if !input.peek(Token![_]) {
                Some(input.parse::<syn::Ident>()?)
            } else {
                input.parse::<Token![_]>()?;
                None
            };

            vec.push((param_name, param_value));

            if !input.peek(Token![,]) {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        Ok(Self { packets: vec })
    }
}

#[proc_macro]
pub fn gamepackets(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as GamepacketsInput);

    let variants = args.packets.clone();
    let variants = variants.iter().map(|(name, value)| {
        if let Some(value) = value {
            quote! {
                #name(#value),
            }
        } else {
            quote! {
                #name(),
            }
        }
    });

    let compress = args.packets.clone();
    let compress = compress.iter().map(|(name, value)| {
        if let Some(v) = value {
            quote! {
                GamePackets::#name(_) => { #v::<::bedrockrs_proto_core::GamePacket>::COMPRESS },
            }
        } else {
            quote! {
                GamePackets::#name() => { todo!("impl GamePackets::{}", stringify!(name)); },
            }
        }
    });

    let encrypt = args.packets.clone();
    let encrypt = encrypt.iter().map(|(name, value)| {
        if let Some(v) = value {
            quote! {
                GamePackets::#name(_) => { #v::<::bedrockrs_proto_core::GamePacket>::ENCRYPT },
            }
        } else {
            quote! {
                GamePackets::#name() => { todo!("impl GamePackets::{}", stringify!(name)); },
            }
        }
    });

    let ser = args.packets.clone();
    let ser = ser.iter().map(|(name, value)| {
        if let Some(v) = value {
            quote! {
                GamePackets::#name(v) => {
                    let buf = Vec::new();
                    
                    #v::<::bedrockrs_proto_core::ProtoCodec>::proto_serialize(v, &mut buf)?;
                    
                    let len = buf.len()
                        .try_into()
                        .map_err(ProtoCodecError::FromIntError)?;
                    
                    write_gamepacket_header(stream, len, v::<::bedrockrs_proto_core::GamePacket>::ID, subclient_sender_id, subclient_target_id)?;
                    
                    stream.write_all(buf.as_slice())?;
                }
            }
        } else {
            quote! {
                GamePackets::#name() => { todo!("impl GamePackets::{}", stringify!(name)) },
            }
        }
    });

    let de = args.packets.clone();
    let de = de.iter().map(|(name, value)| {
        if let Some(v) = value {
            quote! {
                #v::<::bedrockrs_proto_core::GamePacket>::ID => {
                    match #v::<::bedrockrs_proto_core::ProtoCodec>::proto_deserialize($stream) {
                        Ok(v) => GamePackets::#name(v),
                        Err(e) => return Err(e),
                    }
                },
            }
        } else {
            quote! {}
        }
    });
    
    let expanded = quote! {
        #[repr(u16)]
        #[derive(Debug, Clone)]
        pub enum GamePackets {
            #(#variants)*
        }

        impl GamePackets {
            pub fn compress(&self) -> bool {
                match self {
                    #(#compress)*
                }
            }

            pub fn encrypt(&self) -> bool {
                match self {
                    #(#encrypt)*
                }
            }
            
            pub fn pk_serialize(&self, stream: &mut Vec<u8>, subclient_sender_id: u8, subclient_target_id: u8) -> Result<(), ProtoCodecError> {
                match self {
                    #(#ser)*
                };
                
                Ok(())
            }
            
            pub fn pk_deserialize(stream: &mut Cursor<&[u8]>) -> Result<(GamePackets, u8, u8), ProtoCodecError> {
                let (_length, gamepacket_id, subclient_sender_id, subclient_target_id) = read_gamepacket_header(stream)?;
                
                let gamepacket = match gamepacket_id {
                    #(#de)*
                    other => {
                        return Err(ProtoCodecError::InvalidGamePacketID(other));
                    },
                };
                
                Ok((gamepacket, subclient_sender_id, subclient_target_id))
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
