extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse, DeriveInput};
use std::fs::File;
use std::io::prelude::*;
use serde_derive_internals::{ast, Ctxt};

#[proc_macro_attribute]
pub fn tscodegen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // so that we can return it untouched
    let clone = item.clone();

    let input: DeriveInput = parse(clone).expect("couldnt parse, attribute is probably not placed on enum/struct");

    // parse it with serde
    let context = Ctxt::new();
    let container = ast::Container::from_ast(&context, &input);

    println!("TODO: generate code for {}", container.ident);

    match container.data {
        ast::Data::Enum(variants) => {
            println!("enum");

            for v in variants {
                println!("variant {:?}", v.ident)
            }
        }
        ast::Data::Struct(_style, fields) => {
            println!("struct");

            for f in fields {
                println!("variant {:?}", f.ident)
            }
        }
    };

    // otherwise it would fail
    context.check().unwrap();

    // don't forget to create directory
    let mut file = File::create(format!("tscodegen/{}.ts", container.ident)).expect("cannot create file");
    file.write_all(b"// TODO: generate something").expect("cannot write file");

    item
}
