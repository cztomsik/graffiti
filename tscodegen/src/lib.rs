extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::fs::OpenOptions;
use std::io::prelude::*;
use syn::{parse, DeriveInput, Ident, Type};

#[proc_macro_attribute]
pub fn tscodegen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(item.clone())
        .expect("couldn't parse, attribute is probably not placed on enum/struct");

    // 1. prepare intermediate representation
    let name = input.ident.clone();
    let ts_type = match input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let members = variants.iter().map(|v| v.ident.clone()).collect();

            TsType::Enum { name, members }
        }

        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let is_tuple = fields.iter().all(|f| f.ident.is_none());

            match is_tuple {
                true => TsType::TupleStruct {
                    name,
                    types: fields.iter().map(|f| f.ty.clone()).collect(),
                },

                false => TsType::Struct {
                    name: name.clone(),
                    properties: fields
                        .iter()
                        .map(|f| (f.ident.clone().unwrap(), f.ty.clone()))
                        .collect(),
                },
            }
        }

        _ => panic!("unexpected data {:?}", input.data),
    };

    // 2. generate "code" using quasiquotes
    let ts = match ts_type {
        TsType::Enum { name, members } => {
            let write_name = Ident::new(format!("write{}", name).as_ref(), name.span());

            quote! {
                export enum #name {
                    #(#members),*
                }

                export function #write_name(buf: Buffer, value: #name) {
                    buf.writeInt8(0, value)
                }
            }
        }

        TsType::TupleStruct { name, types } => {
            let len = types.len();
            let mk_name = Ident::new(format!("mk{}", name).as_ref(), name.span());
            let write_name = Ident::new(format!("write{}", name).as_ref(), name.span());

            quote! {
                export interface #name {
                    length: #len
                }

                export function #mk_name(todoParams) {
                    return []
                }

                export function #write_name() {
                    // TODO
                }
            }
        }

        TsType::Struct { name, properties } => {
            let properties = properties.iter().map(|(name, ty)| {
                quote! {
                    #name: #ty;
                }
            });
            let mk_name = Ident::new(format!("mk{}", name).as_ref(), name.span());
            let write_name = Ident::new(format!("write{}", name).as_ref(), name.span());

            quote! {
                export interface #name {
                    #(#properties)*
                }

                export function #mk_name() {
                    return {
                        // TODO
                    }
                }

                export function #write_name() {
                    // TODO
                }
            }
        }
    };

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("generated/{}.ts", &input.ident))
        .expect("cannot open/create codegen file");

    write!(file, "{}", ts).expect("couldnt write");

    item
}

#[derive(Debug)]
enum TsType {
    Enum {
        name: Ident,
        members: Vec<Ident>,
    },

    Struct {
        name: Ident,
        properties: Vec<(Ident, Type)>,
    },

    TupleStruct {
        name: Ident,
        types: Vec<Type>,
    }, // TODO: discriminated union (hash with "type")
}
