use pest::iterators::Pair;
use crate::express::parser::Rule;
use crate::express::primitives::Primitive;

#[derive(Debug)]
pub struct DefinedDataType {
    pub name: String,
    pub data_type: Primitive
}

impl DefinedDataType {
    pub fn from_pair(pair: Pair<Rule>) -> DefinedDataType {
        let mut tokens = pair.into_inner();
        let name = tokens.next().expect("DefinedDataType should have a name").as_str().to_owned();
        let data_type = Primitive::from_pair(tokens.next().expect("DefinedDataType should have a underlying data type"));

        DefinedDataType {
            name,
            data_type
        }
    }
}