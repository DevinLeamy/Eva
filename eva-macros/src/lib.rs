extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ShaderStructMacro)]
pub fn shader_struct_methods_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::prelude::ShaderStruct for #name {
            fn as_bytes(&self) -> Option<Vec<u8>> {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                buffer.write(self).ok()?;
                Some(buffer.into_inner())
            }
        }
    };

    TokenStream::from(expanded)
}
