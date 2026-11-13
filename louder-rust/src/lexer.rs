#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum LTok {
    Key(String),
    KeyValueSeparator,
    Value(String),
    UnlabeledField(String),
    FieldSeparator,
    BlockSeparator,
}

pub fn lex(input: String) -> Vec<LTok> {
    vec![]
}
