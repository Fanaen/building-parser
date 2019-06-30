/// Important references for data files:
///  - https://technical.buildingsmart.org/standards/ifc/ifc-schema-specifications/
///  - https://www.cax-if.org/joint_testing_info.html#schemas

#[derive(Parser)]
#[grammar = "express/express.pest"]
pub struct ExpressParser;

pub mod schema;
pub mod primitives;
pub mod data_type;
pub mod entity;
pub mod function;
pub mod rule;
pub mod procedure;
