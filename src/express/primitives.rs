use pest::iterators::Pair;
use crate::express::parser::Rule;

#[derive(Debug)]
pub enum PrimitiveType {
    Real,
    Integer,
    Number,
    String,
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
            "STRING" => PrimitiveType::String,
            "BINARY" => PrimitiveType::Binary,
            "BOOLEAN" => PrimitiveType::Boolean,
            "LOGICAL" => PrimitiveType::Logical,
            token @ _ => panic!("Unknown PrimitiveType value: {}", token)
        }
    }
}

#[derive(Debug)]
pub enum AggregationType {
    List,
    Array,
    Bag,
    Set
}

impl AggregationType {
    pub fn from_pair(pair: Pair<Rule>) -> AggregationType {
        assert!(pair.as_rule() == Rule::aggregation_type);
        match pair.as_str() {
            "LIST" => AggregationType::List,
            "ARRAY" => AggregationType::Array,
            "BAG" => AggregationType::Bag,
            "SET" => AggregationType::Set,
            token @ _ => panic!("Unknown AggregationType value: {}", token)
        }
    }
}

#[derive(Debug)]
pub struct Aggregation {
    pub aggregation_type: AggregationType,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub aggregated_type: PrimitiveType
}

impl Aggregation {
    pub fn from_pair(pair: Pair<Rule>) -> Aggregation {
        let mut tokens = pair.into_inner();
        let aggregation_type_token = tokens.next().expect("Aggregation should have a AggregationType");
        let number_annotation_token = tokens.next().expect("Aggregation should have number annotations ");
        let complex_type_token = tokens.next().expect("Aggregation should specify a target type");

        let (min, max) = Aggregation::extract_numbering_annotations(number_annotation_token);

        Aggregation {
            aggregation_type: AggregationType::from_pair(aggregation_type_token),
            aggregated_type: PrimitiveType::from_pair(complex_type_token),
            min,
            max
        }
    }

    fn extract_numbering_annotations(pair: Pair<Rule>) -> (Option<i32>, Option<i32>) {
        let mut tokens = pair.into_inner();
        let min: Option<i32> = tokens.next().expect("Aggregation numbering should have a name").as_str()
            .parse::<i32>().ok();

        let max: &str = tokens.next().expect("DefinedDataType should have an underlying data type").as_str();
        let max: Option<i32> = match max {
            "?" => None,
            v @ _ => v.parse::<i32>().ok(),
        };

        (min, max)
    }
}

#[derive(Debug)]
pub struct Enumeration {
    pub values: Vec<String>
}

impl Enumeration {
    pub fn from_pair(pair: Pair<Rule>) -> Enumeration {
        Enumeration {
            values: pair.into_inner().map(|t| t.as_str().to_owned()).collect()
        }
    }
}

#[derive(Debug)]
pub struct Select {
    pub values: Vec<String>
}

impl Select {
    pub fn from_pair(pair: Pair<Rule>) -> Select {
        Select {
            values: pair.into_inner().map(|t| t.as_str().to_owned()).collect()
        }
    }
}

#[derive(Debug)]
pub enum ComplexType {
    PrimitiveType(PrimitiveType),
    Aggregation(Aggregation)
}


impl ComplexType {
    pub fn from_pair(pair: Pair<Rule>) -> ComplexType {
        match pair.as_rule() {
            Rule::primitive_type => ComplexType::PrimitiveType(PrimitiveType::from_pair(pair)),
            Rule::aggregation => ComplexType::Aggregation(Aggregation::from_pair(pair)),
            token @ _ => panic!("Unknown ComplexType rule: {:?}", token)
        }
    }
}