extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs::OpenOptions;
use syn::{parse, DeriveInput, Path, Type, TypePath};
//use std::io::prelude::*;
use std::fmt;
use serde::Serialize;
use serde_json;
use std::io::prelude::*;

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
                true => TsType::TupleStruct {
                    name,
                    types: fields
                        .iter()
                        .map(|f| to_ts_type(&f.ty))
                        .collect(),
                },

                false => TsType::Struct {
                    name: name.clone(),
                    properties: fields.iter().map(|f| {
                        let key = f.ident.clone().unwrap().to_string();
                        let ts_type = to_ts_type(&f.ty);

                        (key, ts_type)
                    }).collect(),
                },
            }
        }

        _ => panic!("unexpected data {:?}", input.data),
    };

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("generated/{}.json", &input.ident))
        .expect("cannot open/create codegen file");
    
    file.write_all(&serde_json::to_vec_pretty(&ts_type).unwrap());

    item
}

fn to_ts_type(ty: &syn::Type) -> String {
    match ty {
        Type::Path(TypePath {
                       path: Path { segments, .. },
                       ..
                   }) => {
            let parts: Vec<String> =
                segments.iter().map(|s| s.ident.to_string()).collect();
            parts.join(".")
        }

        _ => panic!("unsupported type {:#?}", ty),
    }
}

#[derive(Serialize, Debug)]
enum TsType {
    Enum {
        name: String,
        members: Vec<String>,
    },

    Struct {
        name: String,
        properties: Vec<(String, String)>,
    },

    TupleStruct {
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
