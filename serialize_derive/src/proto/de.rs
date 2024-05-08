use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

pub fn proto_build_de(data: &Data) -> TokenStream {
    let fields = match *data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("Serialize macro only supports structs"),
    };

    let field_de_calls = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field has no name");
        quote! {
            #field_name: match serialize::proto::de::MCProtoDeserialize::proto_deserialize(cursor) {
                Ok(v) => { v },
                Err(e) => { return Err(e) }
            },
        }
    });

    let expand = quote! {
        #(#field_de_calls)*
    };

    TokenStream::from(expand)
}
