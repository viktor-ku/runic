#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ScriptParser;

pub(crate) type PestRule = Rule;
