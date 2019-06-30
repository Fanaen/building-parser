use crate::express::Rule;
use crate::express::primitives::{Aggregation, Enumeration, PrimitiveType, Select};
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DefinedDataType {
    pub name: String,
    pub data_type: DefinableDataType,
}

impl DefinedDataType {
    pub fn from_pair(pair: Pair<Rule>) -> DefinedDataType {
        let mut tokens = pair.into_inner();
        let name = tokens
            .next()
            .expect("DefinedDataType should have a name")
            .as_str()
            .to_owned();
        let data_type = DefinableDataType::from_pair(
            tokens
                .next()
                .expect("DefinedDataType should have an underlying data type"),
        );

        DefinedDataType { name, data_type }
    }
}

#[derive(Debug, Serialize)]
pub enum DefinableDataType {
    PrimitiveType(PrimitiveType),
    Aggregation(Aggregation),
    Enumeration(Enumeration),
    Select(Select),
    Name(String),
}

impl DefinableDataType {
    pub fn from_pair(pair: Pair<Rule>) -> DefinableDataType {
        match pair.as_rule() {
            Rule::primitive_type => {
                DefinableDataType::PrimitiveType(PrimitiveType::from_pair(pair))
            }
            Rule::aggregation => DefinableDataType::Aggregation(Aggregation::from_pair(pair)),
            Rule::enumeration => DefinableDataType::Enumeration(Enumeration::from_pair(pair)),
            Rule::select => DefinableDataType::Select(Select::from_pair(pair)),
            Rule::name => DefinableDataType::Name(pair.as_str().to_owned()),
            token @ _ => panic!("Unknown ComplexType rule: {:?}", token),
        }
    }
}
