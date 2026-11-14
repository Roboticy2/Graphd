use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum LTok {
    Key(String),
    KeyValueSeparator,
    Value(String),
    UnlabeledField(String),
    FieldSeparator,
    BlockSeparator,
    ConnectionDirective,
    Connection(String),
    Unknown(String),
}

pub fn lex(input: String) -> Vec<LTok> {
    // \n process -> field sep or block sep
    //    -> non whitespace -> field sep collect, field start
    //    -> \n again -> block sep start
    //
    // block sep start -> non whitespace -> block sep collect, field start
    //
    // field start
    //    -> does not start with > -> unlabeled or key value
    //       -> : -> key collect (not including delimeter), key value separator collect, value start
    //       -> \n -> unlabeled field collect (not including newline), \n process
    //    -> starts with > -> collect connection directive
    //       -> \n -> collect connection (not including newline), \n process
    //
    // value start -> \n -> collect value (not including newline), \n process

    let mut collect: Vec<LTok> = vec![];

    collect
}
