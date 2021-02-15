#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct InputParser;

pub(crate) type PestRule = Rule;
