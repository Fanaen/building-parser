use crate::express::Rule;
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Procedure {
    pub name: String,
}

impl Procedure {
    pub fn from_pair(pair: Pair<Rule>) -> Procedure {
        let mut tokens = pair.into_inner();
        let name = tokens
            .next()
            .expect("Procedure should have a name")
            .as_str()
            .to_owned();

        Procedure { name }
    }
}
