use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct};

pub fn build_ser_struct(data_struct: &DataStruct) -> TokenStream {
    quote! {}
}

pub fn build_ser_enum(data_enum: &DataEnum) -> TokenStream {
    quote! {}
}
