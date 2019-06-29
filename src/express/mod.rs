/// Important references for data files:
///  - https://technical.buildingsmart.org/standards/ifc/ifc-schema-specifications/
///  - https://www.cax-if.org/joint_testing_info.html#schemas

use pest::Parser;

#[derive(Parser)]
#[grammar = "express.pest"]
struct IdentParser;

pub fn test() {
    let pairs = IdentParser::parse(Rule::express, include_str!("../data/IFC2X3_TC1.exp")).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        print!("\n{:?}:", pair.as_rule());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            print!(" {}", inner_pair.as_str());
        }
    }

    println!();
}
