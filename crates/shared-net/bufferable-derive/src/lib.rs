use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Bufferable)]
pub fn bufferable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    if let syn::Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            let push_into = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote!(pushed += self.#name.push_into(buf)?;)
            });
            let pull_from_vars = fields.named.iter().map(|field| {
                let name = &field.ident;
                let ty = &field.ty;
                quote!(let #name = <#ty>::pull_from(buf)?;)
            });
            let pull_from_self = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote!(#name)
            });
            let size_in_buffer = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote!(self.#name.size_in_buffer())
            });

            let output = quote!(
                impl Bufferable for #name {
                    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError>{
                        let mut pushed = 0;
                        #(#push_into)*
                        Ok(pushed)
                    }

                    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
                        #(#pull_from_vars)*
                        let result = Self {
                            #(#pull_from_self),*
                        };
                        Ok(result)
                    }

                    fn size_in_buffer(&self) -> usize {
                        #(#size_in_buffer)+*
                    }
                }
            );

            return output.into();
        }
    }

    TokenStream::from(syn::Error::new(name.span(), "Only structs with named fields can derive `Bufferable`").to_compile_error())
}
