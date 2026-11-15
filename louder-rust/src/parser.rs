use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "louder.pest"]
pub struct LParser;
