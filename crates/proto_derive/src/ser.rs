use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, DataStruct, Expr, Fields, Index};

pub fn proto_build_ser_struct(struct_data: &DataStruct) -> TokenStream {
    let fields = &struct_data.fields;

    let expand = match fields {
        Fields::Named(ref fields) => {
            let calls = fields.named.iter().map(|f| {
                let field_name = f.ident.clone();

                let mut quote = None;

                for attr in &f.attrs {
                    if attr.path().is_ident("len_repr") {
                        let int_type: Expr = attr.parse_args().expect(format!("Given attribute meta for field {field_name:?} could not be parsed").as_str());

                        quote = Some(quote! {
                            {
                                let len = #int_type::new(match Vec::len(&self.#field_name).try_into() {
                                    Ok(v) => { v },
                                    Err(e) => { return Err(::bedrockrs_proto_core::error::ProtoCodecError::FromIntError(e.into())) }
                                });

                                match len.write(stream) {
                                    Ok(_) => { },
                                    Err(e) => { return Err(::bedrockrs_proto_core::error::ProtoCodecError::IOError(std::sync::Arc::new(e))) }
                                };

                                for i in &self.#field_name {
                                    match ::bedrockrs_proto_core::ProtoCodec::proto_serialize(i, stream) {
                                        Ok(_) => { },
                                        Err(e) => { return Err(e) }
                                    };
                                }
                            };
                        });
                    }
                }

                match quote {
                    None => {
                        quote! {
                            match ::bedrockrs_proto_core::ProtoCodec::proto_serialize(&self.#field_name, stream) {
                                Ok(_) => { },
                                Err(e) => { return Err(e) }
                            };
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
                    if attr.path().is_ident("len_repr") {
                        let int_type: Expr = attr.parse_args().expect(format!("Given attribute meta for field self.{:?} could not be parsed", index.index).as_str());

                        quote = Some(quote! {
                            {
                                let len = #int_type::new(match Vec::len(&self.#index).try_into() {
                                    Ok(v) => { v },
                                    Err(e) => { return Err(::bedrockrs_proto_core::error::ProtoCodecError::FromIntError(e.into())) }
                                });

                                match len.write(stream) {
                                    Ok(_) => { },
                                    Err(e) => { return Err(::bedrockrs_proto_core::error::ProtoCodecError::IOError(std::sync::Arc::new(e))) }
                                };

                                for i in &self.#index {
                                    ::bedrockrs_proto_core::ProtoCodec::proto_serialize(i, stream)?;
                                }
                            };
                        });
                    }
                }

                match quote {
                    None => {
                        quote! {
                            ::bedrockrs_proto_core::ProtoCodec::proto_serialize(&self.#index, stream)?;
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

pub fn proto_build_ser_enum(
    enum_data: &DataEnum,
    attributes: &Vec<Attribute>,
    enum_name: &Ident,
) -> TokenStream {
    let mut int_type: Option<Expr> = None;

    for attr in attributes {
        if attr.path().is_ident("enum_repr") {
            int_type = Some(
                attr.parse_args()
                    .expect(format!("Given attribute meta for enum could not be parsed").as_str()),
            );
        }
    }

    let int_type = int_type
        .unwrap_or_else(|| panic!("Missing attribute \"enum_repr\" for ProtoCodec macro on Enum"));

    let calls = enum_data.variants.iter().map(|v| {
        let val = v.discriminant.clone().expect("Discriminant needed").1;
        let name = v.ident.clone();

        quote! {
            #enum_name::#name => #val,
        }
    });

    quote! {
        let int = #int_type::new(match self {
            #(#calls)*
        });

        match int.write(stream) {
            Ok(_) => { },
            Err(e) => { return Err(::bedrockrs_proto_core::error::ProtoCodecError::IOError(std::sync::Arc::new(e))) }
        };
    }
}
