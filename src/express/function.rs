use crate::express::Rule;
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Function {
    pub name: String,
}

impl Function {
    pub fn from_pair(pair: Pair<Rule>) -> Function {
        let mut tokens = pair.into_inner();
        let name = tokens
            .next()
            .expect("Function should have a name")
            .as_str()
            .to_owned();

        Function { name }
    }
}
