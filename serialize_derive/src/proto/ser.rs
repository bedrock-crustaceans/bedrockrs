use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

pub fn proto_build_ser(data: &Data) -> TokenStream {
    let fields = match *data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("Serialize macro only supports structs"),
    };

    let field_ser_calls = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field has no name");
        quote! {
            match serialize::proto::ser::MCProtoSerialize::proto_serialize(&self.#field_name, buf) {
                Ok(_) => { },
                Err(e) => { return Err(e) }
            };
        }
    });

    let expand = quote! {
        #(#field_ser_calls)*
    };

    TokenStream::from(expand)
}
