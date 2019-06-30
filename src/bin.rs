extern crate pest;
#[macro_use]
extern crate pest_derive;

mod express;

use crate::express::schema::{Schema, SchemaStats};
use express::parser::parse;

pub fn main() {
    // STEP
    assert_completeness(
        parse(include_str!("../data/ap209ed2_is_mim_lf_v1.46.exp")),
        "ap209ed2_is_mim_lf_v1.46.exp",
        SchemaStats {
            entities: 2_225,
            defined_data_types: 555,
        },
    );
    assert_completeness(
        parse(include_str!("../data/AP214E3_2010.exp")),
        "AP214E3_2010.36.exp",
        SchemaStats {
            entities: 915,
            defined_data_types: 192,
        },
    );
    assert_completeness(
        parse(include_str!("../data/ap242_is_mim_lf_v1.36.exp")),
        "ap242_is_mim_lf_v1.36.exp",
        SchemaStats {
            entities: 1_726,
            defined_data_types: 370,
        },
    );

    // IFC
    assert_completeness(
        parse(include_str!("../data/IFC2X3_TC1.exp")),
        "IFC2X3_TC1.exp",
        SchemaStats {
            entities: 653,
            defined_data_types: 327,
        },
    );
    assert_completeness(
        parse(include_str!("../data/IFC4.exp")),
        "IFC4.exp",
        SchemaStats {
            entities: 776,
            defined_data_types: 397,
        },
    );
    assert_completeness(
        parse(include_str!("../data/IFC4x1.exp")),
        "IFC4x1.exp",
        SchemaStats {
            entities: 801,
            defined_data_types: 400,
        },
    );
    assert_completeness(
        parse(include_str!("../data/IFC4x2.exp")),
        "IFC4x2.exp",
        SchemaStats {
            entities: 816,
            defined_data_types: 407,
        },
    );
}

fn assert_completeness(schemas: Vec<Schema>, name: &str, expected: SchemaStats) {
    println!("{:=>20}: {}", " File", name);

    // There is one schema per file
    match schemas.len() {
        0 => println!("{: >20}: {}", " Error", "No valid schema matched"),
        1 => {
            let schema = schemas.first().unwrap();

            // Compare the count and expected
            let stats = schema.stats();
            stats.display_completeness(&expected);
        }
        _ => println!("{: >20}: {}", " Error", "Too many schema matched"),
    }

    println!();
}
