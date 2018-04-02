#![crate_type = "proc-macro"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field, Fields, Ident};
use quote::Tokens;

#[proc_macro_derive(Withers)]
pub fn withers_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("Couldn't parse input");

    let gen = match ast.data {
        Data::Struct(ref data) => impl_withers(&ast, &data.fields),
        // TODO: add to properties as well
        _ => panic!("#[derive(Withers)] must be used on a struct"),
    };

    gen.into()
}

fn impl_withers(ast: &DeriveInput, fields: &Fields) -> Tokens {
    let name = &ast.ident;
    match fields {
        &Fields::Named(ref fields) => {
            let fields: Vec<&Field> = fields.named.iter().collect();
            let idents: Vec<_> = fields
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    (Ident::from(format!("with_{}", ident)), ident, &f.ty)
                })
                .map(|(with_name, name, ty)| (quote!(#with_name), quote!(#name), quote!(#ty)))
                .collect();

            let withers = idents.iter().map(|&(ref with_name, ref name, ref ty)| {
                quote! {
                    /// Sets the #name property.
                    pub fn #with_name(mut self, value: #ty) -> Self {
                        self.#name = value;
                        self
                    }
                }
            });

            quote! {
                #[allow(dead_code)]
                impl #name {
                    #(#withers)*
                }
            }
        }
        _ => panic!(
            "#[derive(Withers)] cannot be used on tuple struct: {}",
            name
        ),
    }
}
