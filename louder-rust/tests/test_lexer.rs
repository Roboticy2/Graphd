use louder_rust::lexer::*;
use vector_assertions::assert_vec_eq;

#[test]
fn test_separators() {
    let test = "

        %opening block separator

        %separator doesnt end until there is a non-comment
        non comment
        field separator before next non comment

        block separator before bext object
        %block separator interrupted by comment

        last object
    "
    .to_string();

    // expected lex: block separator, unlabeled field, field separator, unlabaled field, block separator, unlabeled field, block separator, unlabeled field, field separator (for the final newline)
    // the lexer does not do any special syntax, even where it would make sense, like omitting the final field separator because nothing comes after it.

    let tokens = lex(test);
    assert_vec_eq!(
        tokens,
        vec![
            LTok::BlockSeparator,
            LTok::UnlabeledField("non comment".to_string()),
            LTok::FieldSeparator,
            LTok::UnlabeledField("field separator before next non comment".to_string()),
            LTok::BlockSeparator,
            LTok::UnlabeledField("block separator before bext object".to_string()),
            LTok::BlockSeparator,
            LTok::UnlabeledField("last object".to_string()),
            LTok::FieldSeparator
        ]
    );
}
