use logos::Logos;

#[derive(Logos, PartialEq, Eq, Clone, Debug, Hash)]
#[logos(skip(r"[ \t\f]+", priority = 3))]
pub enum LTok {
    #[regex(r"[^%>\n: ][^%:\n]*")]
    Text,
    #[token(":")]
    KeyValueSeparator,
    #[token("\n")]
    Separator,
    #[token(">")]
    ConnectionDirective,
    #[token("%")]
    Comment,
}