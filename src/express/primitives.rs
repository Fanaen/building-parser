use crate::express::Rule;
use pest::iterators::Pair;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StringType {
    max_length: Option<i32>,
    fixed_length: bool,
}

#[derive(Debug, Serialize)]
pub enum PrimitiveType {
    Real,
    Integer,
    Number,
    String(StringType),
    Binary,
    Boolean,
    Logical,
}

impl PrimitiveType {
    pub fn from_pair(pair: Pair<Rule>) -> PrimitiveType {
        assert!(pair.as_rule() == Rule::primitive_type);

        match pair.as_str() {
            "REAL" => PrimitiveType::Real,
            "INTEGER" => PrimitiveType::Integer,
            "NUMBER" => PrimitiveType::Number,
            "STRING" => PrimitiveType::String(StringType { max_length: None, fixed_length: false}),
            "BINARY" => PrimitiveType::Binary,
            "BOOLEAN" => PrimitiveType::Boolean,
            "LOGICAL" => PrimitiveType::Logical,
            token @ _ => {
                // Handle the "STRING(22) FIXED" special case
                let mut tokens = pair.into_inner();
                if let Option::Some(sub_token) = tokens.next() {
                    let max_length = sub_token.as_str().parse::<i32>().ok();
                    let fixed_length = tokens.next().is_some();

                    return PrimitiveType::String(StringType {
                        max_length,
                        fixed_length,
                    });
                } else {
                    panic!("Unknown PrimitiveType value: {}", token)
                }
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum AggregationType {
    List,
    Array,
    Bag,
    Set,
}

impl AggregationType {
    pub fn from_pair(pair: Pair<Rule>) -> AggregationType {
        assert!(pair.as_rule() == Rule::aggregation_type);
        match pair.as_str() {
            "LIST" => AggregationType::List,
            "ARRAY" => AggregationType::Array,
            "BAG" => AggregationType::Bag,
            "SET" => AggregationType::Set,
            token @ _ => panic!("Unknown AggregationType value: {}", token),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Aggregation {
    pub aggregation_type: AggregationType,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub aggregated_type: PrimitiveOrNamedType,
}

impl Aggregation {
    pub fn from_pair(pair: Pair<Rule>) -> Aggregation {
        let mut tokens = pair.into_inner();
        let aggregation_type_token = tokens
            .next()
            .expect("Aggregation should have a AggregationType");
        let number_annotation_token = tokens
            .next()
            .expect("Aggregation should have number annotations ");
        let complex_type_token = tokens
            .next()
            .expect("Aggregation should specify a target type");

        let (min, max) = Aggregation::extract_numbering_annotations(number_annotation_token);

        Aggregation {
            aggregation_type: AggregationType::from_pair(aggregation_type_token),
            aggregated_type: PrimitiveOrNamedType::from_pair(complex_type_token),
            min,
            max,
        }
    }

    fn extract_numbering_annotations(pair: Pair<Rule>) -> (Option<i32>, Option<i32>) {
        let mut tokens = pair.into_inner();
        let min: Option<i32> = tokens.next().and_then(|v| v.as_str().parse::<i32>().ok());
        let max: Option<i32> = tokens.next().and_then(|v| match v.as_str() {
            "?" => None,
            v @ _ => v.parse::<i32>().ok(),
        });

        (min, max)
    }
}

#[derive(Debug, Serialize)]
pub struct Enumeration {
    pub values: Vec<String>,
}

impl Enumeration {
    pub fn from_pair(pair: Pair<Rule>) -> Enumeration {
        Enumeration {
            values: pair.into_inner().map(|t| t.as_str().to_owned()).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Select {
    pub values: Vec<String>,
}

impl Select {
    pub fn from_pair(pair: Pair<Rule>) -> Select {
        Select {
            values: pair.into_inner().map(|t| t.as_str().to_owned()).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum PrimitiveOrNamedType {
    PrimitiveType(PrimitiveType),
    Name(String),
}

impl PrimitiveOrNamedType {
    pub fn from_pair(pair: Pair<Rule>) -> PrimitiveOrNamedType {
        match pair.as_rule() {
            Rule::primitive_type => {
                PrimitiveOrNamedType::PrimitiveType(PrimitiveType::from_pair(pair))
            }
            _ => PrimitiveOrNamedType::Name(pair.as_str().to_owned()),
        }
    }
}
