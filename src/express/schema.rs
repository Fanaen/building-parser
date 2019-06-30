use crate::express::data_type::DefinedDataType;
use crate::express::entity::Entity;
use crate::express::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
    pub defined_data_types: Vec<DefinedDataType>,
}

impl Schema {
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
        };

        for token in tokens {
            match token.as_rule() {
                Rule::entity => schema.entities.push(Entity::from_pair(token)),
                Rule::defined_data_type => schema
                    .defined_data_types
                    .push(DefinedDataType::from_pair(token)),
                Rule::unparsed => {
                    // print!("{}", pair.as_str());
                }
                _ => {
                    // A pair is a combination of the rule which matched and a span of input
                    print!("\n{:?}:", token.as_rule());

                    // A pair can be converted to an iterator of the tokens which make it up:
                    for inner_pair in token.into_inner() {
                        print!(" {}", inner_pair.as_str());
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
        }
    }
}

/// Struct used to assert the completeness of the schema
pub struct SchemaStats {
    pub entities: usize,
    pub defined_data_types: usize,
}

impl SchemaStats {
    pub fn display_completeness(&self, expected: &SchemaStats) {
        display_stat("Entity", self.entities, expected.entities);
        display_stat(
            "Defined data types",
            self.defined_data_types,
            expected.defined_data_types,
        );
    }
}

fn display_stat(name: &str, count: usize, expected: usize) {
    println!(
        "{: >20}: {: >6} {: >5} / {: <5}",
        name,
        format!("{:.1}%", count as f32 / expected as f32 * 100.),
        count,
        expected
    );
}
