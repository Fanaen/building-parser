use crate::express::schema::Schema;
//use proc_macro::{Ident, Span};

pub trait Generate {
    fn generate(&self);
}

impl Generate for Schema {
    fn generate(&self) {
//        let ident = Ident::new("demo", Span::call_site());
//        let tokens = quote! {
//            struct Test {
//                test: i32;
//            }
//        };

        // println!("{}", tokens);
    }
}
