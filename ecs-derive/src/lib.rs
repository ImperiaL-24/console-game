extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse::Parser, DeriveInput, Ident};

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    // let args_parsed = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated.parse(args).unwrap();
    // for arg in args_parsed {
    //     if arg.get_ident().unwrap() == &syn::parse_str::<Ident>("Value").unwrap() {}
    // }
    let mut ast = syn::parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! { name: String }).unwrap());
                }
                _ => (),
            }
            quote! {
                #ast
                impl Named for #name {
                    fn name(&self) -> String {
                        self.name.clone()
                    }
                }
            }
            .into()
        }
        _ => panic!("`entity` has to be used with structs "),
    }
}
