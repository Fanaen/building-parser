extern crate pest;
#[macro_use]
extern crate pest_derive;

mod express;

use express::parser::parse;

pub fn main() {
    // STEP
    parse(include_str!("../data/ap209ed2_is_mim_lf_v1.46.exp"), "ap209ed2_is_mim_lf_v1.46.exp");
    parse(include_str!("../data/AP214E3_2010.exp"), "AP214E3_2010.36.exp");
    parse(include_str!("../data/ap242_is_mim_lf_v1.36.exp"), "ap242_is_mim_lf_v1.36.exp");

    // IFC
    parse(include_str!("../data/IFC2X3_TC1.exp"), "IFC2X3_TC1.exp");
    parse(include_str!("../data/IFC4.exp"), "IFC4.exp");
    parse(include_str!("../data/IFC4x1.exp"), "IFC4x1.exp");
    parse(include_str!("../data/IFC4x2.exp"), "IFC4x2.exp");
}