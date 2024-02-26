extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use std::collections::HashSet;

use proc_macro::{Span, TokenStream};
use syn::{parse::Parser, DeriveInput, Ident};

#[derive(Hash, Eq, PartialEq)]
enum ComponentType {
    POS,
    ROT,
    SCALE,
    TRANSFORM,
}

#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_parsed = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated.parse(args).unwrap();
    let mut args_simple: HashSet<ComponentType> = HashSet::new();
    for arg in args_parsed {
        if arg.get_ident().unwrap() == &Ident::new("Pos", Span::call_site().into()) {
            args_simple.insert(ComponentType::POS);
        }
        if arg.get_ident().unwrap() == &Ident::new("Rot", Span::call_site().into()) {
            args_simple.insert(ComponentType::ROT);
        }
        if arg.get_ident().unwrap() == &Ident::new("Scale", Span::call_site().into()) {
            args_simple.insert(ComponentType::SCALE);
        }
        if arg.get_ident().unwrap() == &Ident::new("Transform", Span::call_site().into()) {
            args_simple.insert(ComponentType::TRANSFORM);
        }
    }

    let mut ast = syn::parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            let has_pos = args_simple.contains(&ComponentType::POS) || args_simple.contains(&ComponentType::TRANSFORM);
            let has_rot = args_simple.contains(&ComponentType::ROT) || args_simple.contains(&ComponentType::TRANSFORM);
            let has_scale = args_simple.contains(&ComponentType::SCALE) || args_simple.contains(&ComponentType::TRANSFORM);
            let has_transform = has_pos || has_rot || has_scale;

            /* Adding in new fields to the struct */
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! { name: String }).unwrap());
                    if has_pos {
                        fields.named.push(syn::Field::parse_named.parse2(quote! { pos: Vec3 }).unwrap());
                    }
                    if has_rot {
                        fields.named.push(syn::Field::parse_named.parse2(quote! { rot: Vec3 }).unwrap());
                    }
                    if has_scale {
                        fields.named.push(syn::Field::parse_named.parse2(quote! { scale: f64 }).unwrap());
                    }
                }
                _ => (),
            }

            let pos_impl = has_pos.then(|| {
                quote! {
                    impl Pos for #name {
                        fn pos(&self) -> Vec3 {
                            self.pos.clone()
                        }
                        fn set_pos(&mut self, new_pos: Vec3) {
                            self.pos = new_pos;
                        }
                    }
                }
            });

            let rot_impl = has_rot.then(|| {
                quote! {
                    impl Rot for #name {
                        fn rot(&self) -> Vec3 {
                            self.rot.clone()
                        }
                        fn set_rot(&mut self, new_rot: Vec3) {
                            self.rot = new_rot;
                        }
                    }
                }
            });

            let scale_impl = has_scale.then(|| {
                quote! {
                    impl Scale for #name {
                        fn scale(&self) -> f64 {
                            self.scale
                        }
                        fn set_scale(&mut self, new_scale: f64) {
                            self.scale = new_scale;
                        }
                    }
                }
            });

            let trans_pos = has_pos.then(|| {
                quote! {
                    transform[(0,3)] = self.pos[0];
                    transform[(1,3)] = self.pos[1];
                    transform[(2,3)] = self.pos[2];
                }
            });
            //TODO: implement the real rotation
            let trans_rot = has_rot.then(|| {
                quote! {
                    transform[(0,0)] *= self.rot[0];
                    transform[(1,1)] *= self.rot[1];
                    transform[(2,2)] *= self.rot[2];
                }
            });
            let trans_scale = has_scale.then(|| {
                quote! {
                    transform[(0,0)] *= self.scale;
                    transform[(1,1)] *= self.scale;
                    transform[(2,2)] *= self.scale;
                }
            });

            let trans_impl = has_transform.then(|| {
                quote! {
                    impl Transform for #name {
                        fn as_transform(&self) -> TransformMatrix {
                            let mut transform = TransformMatrix::new();
                            #trans_pos
                            #trans_rot
                            #trans_scale
                            transform
                        }
                    }
                }
            });
            quote! {
                #ast
                impl Named for #name {
                    fn name(&self) -> String {
                        self.name.clone()
                    }
                }
                #pos_impl
                #rot_impl
                #scale_impl
                #trans_impl
            }
            .into()
        }

        _ => panic!("`entity` has to be used with structs "),
    }
}
