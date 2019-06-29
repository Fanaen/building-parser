use pest::iterators::Pair;
use crate::express::parser::Rule;

#[derive(Debug)]
pub enum Primitive {
    Real,
    Integer,
    Number,
    String,
    Binary,
    Boolean,
    Logical,
}

impl Primitive {
    pub fn from_pair(pair: Pair<Rule>) -> Primitive {
        assert!(pair.as_rule() == Rule::primitive_type);
        match pair.as_str() {
            "REAL" => Primitive::Real,
            "INTEGER" => Primitive::Integer,
            "NUMBER" => Primitive::Number,
            "STRING" => Primitive::String,
            "BINARY" => Primitive::Binary,
            "BOOLEAN" => Primitive::Boolean,
            "LOGICAL" => Primitive::Logical,
            token @ _ => panic!("Unknown primitive type: {}", token)
        }
    }
}