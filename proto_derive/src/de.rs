use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, Expr, Fields, Index};

pub fn proto_build_de_struct(struct_data: &DataStruct) -> TokenStream {
    let fields = &struct_data.fields;

    let expand = match fields {
        Fields::Named(ref fields) => {
            let calls = fields.named.iter().map(|f| {
                let field_name = f.ident.clone();

                let mut quote = None;

                for attr in &f.attrs {
                    if attr.path().is_ident("len_type") {
                        let int_type: Expr = attr.parse_args().expect(format!("Given attribute meta for field {field_name:?} could not be parsed").as_str());

                        quote = Some(quote! {
                            #field_name: {
                                let len = match #int_type::read(stream) {
                                    Ok(v) => { v.into_inner() },
                                    Err(e) => { return Err(bedrockrs_proto_core::error::ProtoCodecError::IOError(std::sync::Arc::new(e))) }
                                };

                                let mut vec = Vec::with_capacity(match len.try_into() {
                                    Ok(v) => { v },
                                    Err(e) => { return Err(bedrockrs_proto_core::error::ProtoCodecError::FromIntError(e.into())) }
                                });

                                for _ in 0..len {
                                    vec.push(match bedrockrs_proto_core::ProtoCodec::proto_deserialize(stream) {
                                        Ok(v) => { v },
                                        Err(e) => { return Err(e) }
                                    });
                                };

                                vec
                            },
                        });
                    }
                }

                match quote {
                    None => {
                        quote! {
                            #field_name: match bedrockrs_proto_core::ProtoCodec::proto_deserialize(stream) {
                                Ok(v) => { v },
                                Err(e) => { return Err(e) }
                            },
                        }
                    }
                    Some(v) => {
                        v
                    }
                }
            });

            quote! {
                #(#calls)*
            }
        }
        Fields::Unnamed(ref fields) => {
            let calls = fields.unnamed.iter().enumerate().map(|(i, f)| {
                let index = Index::from(i);

                let mut quote = None;

                for attr in &f.attrs {
                    if attr.path().is_ident("len_type") {
                        let int_type: Expr = attr.parse_args().expect(format!("Given attribute meta for field self.{index:?} could not be parsed").as_str());

                        quote = Some(quote! {
                            #index: {
                                let len = match #int_type::read(stream) {
                                    Ok(v) => { v.into_inner() },
                                    Err(e) => { return Err(bedrockrs_proto_core::error::ProtoCodecError::IOError(std::sync::Arc::new(e))) }
                                };

                                let mut vec = Vec::with_capacity( match len.try_into() {
                                    Ok(v) => { v },
                                    Err(e) => { return Err(bedrockrs_proto_core::error::ProtoCodecError::FromIntError(e.into())) }
                                });

                                for _ in 0..len {
                                    vec.push(match bedrockrs_proto_core::ProtoCodec::proto_deserialize(stream) {
                                        Ok(v) => { v },
                                        Err(e) => { return Err(e) }
                                    });
                                };

                                vec
                            },
                        });
                    }
                }

                match quote {
                    None => {
                        quote! {
                            #index: match bedrockrs_proto_core::ProtoCodec::proto_deserialize(stream) {
                                Ok(v) => { v },
                                Err(e) => { return Err(e) }
                            },
                        }
                    }
                    Some(v) => {
                        v
                    }
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
