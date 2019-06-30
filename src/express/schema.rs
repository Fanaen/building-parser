use crate::express::entity::Entity;
use crate::express::data_type::DefinedDataType;
use crate::express::function::Function;
use crate::express::{Rule, ExpressParser};
use pest::Parser;
use pest::iterators::Pair;
use serde::Serialize;
use crate::express::rule::ExpressRule;
use crate::express::procedure::Procedure;

#[derive(Debug, Serialize)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
    pub defined_data_types: Vec<DefinedDataType>,
    pub functions: Vec<Function>,
    pub rules: Vec<ExpressRule>,
    pub procedures: Vec<Procedure>,
}

impl Schema {
    pub fn extract_from_str(file_content: &str) -> Vec<Schema> {
        let pairs = ExpressParser::parse(Rule::express, file_content).unwrap_or_else(|e| panic!("{}", e));

        let mut schemas: Vec<Schema> = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::schema => schemas.push(Schema::from_pair(pair)),
                token @ _ => error!("Unhandled rule '{:?}'", token),
            }
        }

        schemas
    }

    pub fn from_pair(pair: Pair<Rule>) -> Schema {
        let mut tokens = pair.into_inner();
        let name = tokens
            .next()
            .expect("Schema should have a name")
            .as_str()
            .to_owned();

        let mut schema = Schema {
            name,
            entities: Vec::new(),
            defined_data_types: Vec::new(),
            functions: Vec::new(),
            rules: Vec::new(),
            procedures: Vec::new(),
        };

        for token in tokens {
            match token.as_rule() {
                Rule::entity => schema.entities.push(Entity::from_pair(token)),
                Rule::defined_data_type => schema
                    .defined_data_types
                    .push(DefinedDataType::from_pair(token)),
                Rule::function => schema.functions.push(Function::from_pair(token)),
                Rule::rule => schema.rules.push(ExpressRule::from_pair(token)),
                Rule::procedure => schema.procedures.push(Procedure::from_pair(token)),
                Rule::constant => debug!("Found a CONSTANT block"),
                Rule::unparsed => debug!("{}", token.as_str()),
                _ => {
                    // A pair is a combination of the rule which matched and a span of input
                    info!("\n{:?}:", token.as_rule());

                    // A pair can be converted to an iterator of the tokens which make it up:
                    for inner_pair in token.into_inner() {
                        info!(" {}", inner_pair.as_str());
                    }
                }
            }
        }

        schema
    }

    pub fn stats(&self) -> SchemaStats {
        SchemaStats {
            entities: self.entities.len(),
            defined_data_types: self.defined_data_types.len(),
            functions: self.functions.len(),
            rules: self.rules.len(),
            procedures: self.procedures.len(),
        }
    }
}

/// Struct used to assert the completeness of the schema
pub struct SchemaStats {
    pub entities: usize,
    pub defined_data_types: usize,
    pub functions: usize,
    pub rules: usize,
    pub procedures: usize,
}

impl SchemaStats {
    pub fn display_completeness(&self, expected: &SchemaStats) {
        display_stat("Entities", self.entities, expected.entities);
        display_stat(
            "Defined data types",
            self.defined_data_types,
            expected.defined_data_types,
        );
        display_stat("Functions", self.functions, expected.functions);
        display_stat("Rules", self.rules, expected.rules);
        display_stat("Procedures", self.procedures, expected.procedures);
    }
}

fn display_stat(name: &str, count: usize, expected: usize) {
    if count == 0 && expected == 0 { return; }

    println!(
        "{: >20}: {: >6} {: >5} / {: <5}",
        name,
        format!("{:.1}%", count as f32 / expected as f32 * 100.),
        count,
        expected
    );
}
