use pest::iterators::Pair;
use crate::express::parser::Rule;

#[derive(Debug)]
pub struct Schema {
    pub name: String
}

impl Schema {
    pub fn from_pair(pair: Pair<Rule>) -> Schema {
        let mut tokens = pair.into_inner();
        let name = tokens.next().expect("Schema should have a name").as_str().to_owned();

        Schema {
            name
        }
    }
}