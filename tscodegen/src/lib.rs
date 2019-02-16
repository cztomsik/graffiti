extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs::OpenOptions;
use syn::{parse, DeriveInput, Ident, Path, Type, TypePath};
//use std::io::prelude::*;
use std::fmt;

#[proc_macro_attribute]
pub fn tscodegen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(item.clone())
        .expect("couldn't parse, attribute is probably not placed on enum/struct");

    let name = input.ident.to_string();

    let ts_type = match input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let members = variants.iter().map(|v| v.ident.to_string()).collect();

            TsType::Enum { name, members }
        }

        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let is_tuple = fields.iter().all(|f| f.ident.is_none());

            match is_tuple {
                true => TsType::Tuple {
                    name,
                    types: fields
                        .iter()
                        .map(|f| match f.ty.clone() {
                            Type::Path(TypePath {
                                path: Path { segments, .. },
                                ..
                            }) => {
                                let parts: Vec<String> =
                                    segments.iter().map(|s| s.ident.to_string()).collect();
                                parts.join(".")
                            }

                            _ => panic!("unsupported type {:#?}", f.ty),
                        })
                        .collect(),
                },

                false => TsType::Interface {
                    name: name.clone(),
                    properties: vec![(name.clone(), name.clone())],
                },
            }
        }

        _ => panic!("unexpected data {:?}", input.data),
    };

    let mut _file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("tscodegen.ts")
        .expect("cannot open/create codegen file");

    match ts_type {
        TsType::Enum { name, members } => {
            println!("enum {} {{ ... }}", name);
            println!("{:#?}", members);
        }

        TsType::Interface { name, properties } => {
            println!("interface {} {{ ... }}", name);
            println!("{:#?}", properties);
        }

        TsType::Tuple { name, types } => {
            println!("type {} = [...]", name);
            println!("{:#?}", types);
        }
    }

    item
}

#[derive(Debug)]
enum TsType {
    Enum {
        name: String,
        members: Vec<String>,
    },

    Interface {
        name: String,
        properties: Vec<(String, String)>,
    },

    Tuple {
        name: String,
        types: Vec<String>,
    }, // TODO: discriminated union (hash with "type")
}

struct TsIdent(String);

impl fmt::Debug for TsIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}
