extern crate pest;
#[macro_use]
extern crate pest_derive;
//#[macro_use]
extern crate quote;
//#[macro_use]
extern crate syn;
extern crate proc_macro;
#[macro_use]
extern crate log;
extern crate env_logger;

mod express;
mod generation;

use crate::express::schema::{SchemaStats, Schema};
use crate::generation::Generate;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use std::fs;

pub fn main() -> std::io::Result<()> {
    env_logger::init();

    // STEP
    assert_completeness(
        "data/ap209ed2_is_mim_lf_v1.46.exp",
        SchemaStats {
            entities: 2_225,
            defined_data_types: 555,
            functions: 310,
            rules: 57,
            procedures: 7,
        },
    )?;
    assert_completeness(
        "data/AP214E3_2010.exp",
        SchemaStats {
            entities: 915,
            defined_data_types: 192,
            functions: 114,
            rules: 272,
            procedures: 0,
        },
    )?;
    assert_completeness(
        "data/ap242_is_mim_lf_v1.36.exp",
        SchemaStats {
            entities: 1_726,
            defined_data_types: 370,
            functions: 280,
            rules: 57,
            procedures: 7,
        },
    )?;

    // IFC
    assert_completeness(
        "data/IFC2X3_TC1.exp",
        SchemaStats {
            entities: 653,
            defined_data_types: 327,
            functions: 38,
            rules: 2,
            procedures: 0,
        },
    )?;
    assert_completeness(
        "data/IFC4.exp",
        SchemaStats {
            entities: 776,
            defined_data_types: 397,
            functions: 47,
            rules: 2,
            procedures: 0,
        },
    )?;
    assert_completeness(
        "data/IFC4x1.exp",
        SchemaStats {
            entities: 801,
            defined_data_types: 400,
            functions: 47,
            rules: 2,
            procedures: 0,
        },
    )?;
    assert_completeness(
        "data/IFC4x2.exp",
        SchemaStats {
            entities: 816,
            defined_data_types: 407,
            functions: 47,
            rules: 2,
            procedures: 0,
        },
    )
}

fn assert_completeness(path: &str, expected: SchemaStats) -> std::io::Result<()> {
    // Display the name of the file
    let filename = Path::new(path)
        .file_stem()
        .and_then(|v| v.to_str())
        .unwrap_or(path);
    println!("{:=>20}: {}", " File", filename);

    // Open it and display its size
    let file = File::open(path)?;
    let size = file.metadata()?.len() as f32 / 1_000_000.;
    println!("{: >20}: {:.2} Mo", " File size", size);

    // Read it
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // Parse it
    let schemas = Schema::extract_from_str(contents.as_str());

    // Analyse it
    // There is one schema per file
    match schemas.len() {
        1 => process_schema(schemas.first().unwrap(), filename, expected)?,
        0 => println!("{: >20}: {}", " Error", "No valid schema matched"),
        _ => println!("{: >20}: {}", " Error", "Too many schema matched"),
    }

    // Add a line to avoid packed text
    println!();

    Ok(())
}

fn process_schema(schema: &Schema, filename: &str, expected: SchemaStats) -> std::io::Result<()> {
    // Compare the count and expected
    let stats = schema.stats();
    stats.display_completeness(&expected);

    // Generate the schema as JSON
    let serialized = serde_json::to_string_pretty(&schema).unwrap();
    fs::write(format!("data/generated/{}.json", filename), serialized).expect("Unable to write file");

    // Generate the types in Rust
    schema.generate();

    Ok(())
}
