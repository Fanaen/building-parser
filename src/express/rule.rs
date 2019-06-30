use crate::express::Rule;
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExpressRule {
    pub name: String,
}

impl ExpressRule {
    pub fn from_pair(pair: Pair<Rule>) -> ExpressRule {
        let mut tokens = pair.into_inner();
        let name = tokens
            .next()
            .expect("Rule should have a name")
            .as_str()
            .to_owned();

        ExpressRule { name }
    }
}
