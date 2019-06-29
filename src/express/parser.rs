use pest::Parser;
use crate::express::schema::Schema;
use crate::express::data_type::DefinedDataType;
use crate::express::entity::Entity;

#[derive(Parser)]
#[grammar = "express/express.pest"]
pub struct ExpressParser;

pub fn parse() {
    let pairs = ExpressParser::parse(Rule::express, include_str!("../../data/IFC2X3_TC1.exp"))
        .unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        match pair.as_rule() {
            Rule::schema => println!("{:?}", Schema::from_pair(pair)),
            Rule::entity => println!("{:?}", Entity::from_pair(pair)),
            Rule::defined_data_type => println!("{:?}", DefinedDataType::from_pair(pair)),
            _ => {
                // A pair is a combination of the rule which matched and a span of input
                print!("\n{:?}:", pair.as_rule());

                // A pair can be converted to an iterator of the tokens which make it up:
                for inner_pair in pair.into_inner() {
                    print!(" {}", inner_pair.as_str());
                }
            }
        }
    }

    println!();
}
