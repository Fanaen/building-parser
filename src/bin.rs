extern crate pest;
#[macro_use]
extern crate pest_derive;

mod express;

use express::parser::parse;

pub fn main() {
    parse();
}