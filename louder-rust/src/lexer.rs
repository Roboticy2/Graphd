#[derive(PartialEq, Clone, Debug)]
pub enum LTok {
    Key(String),
    KeyValueSeparator,
    Comment,
    Value(String),
    UnlabeledValue(String),
    EntrySeparator,
    BlockSeparator,
}
