use crate::express::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Entity {
    pub name: String,
}

impl Entity {
    pub fn from_pair(pair: Pair<Rule>) -> Entity {
        let mut tokens = pair.into_inner();
        let name = tokens
            .next()
            .expect("Entity should have a name")
            .as_str()
            .to_owned();

        Entity { name }
    }
}
