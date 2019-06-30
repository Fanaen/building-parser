use crate::express::schema::Schema;
use pest::Parser;

#[derive(Parser)]
#[grammar = "express/express.pest"]
pub struct ExpressParser;

pub fn parse(file: &str) -> Vec<Schema> {
    let pairs = ExpressParser::parse(Rule::express, file).unwrap_or_else(|e| panic!("{}", e));

    let mut schemas: Vec<Schema> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::schema => schemas.push(Schema::from_pair(pair)),
            token @ _ => println!("Unhandled rule '{:?}'", token),
        }
    }

    schemas
}
