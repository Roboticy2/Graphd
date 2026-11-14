use logos::Logos;
use louder_rust::lexer::*;

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
    ";

    // expected lex: block separator, unlabeled field, field separator, unlabaled field, block separator, unlabeled field, block separator, unlabeled field, field separator (for the final newline)
    // the lexer does not do any special syntax, even where it would make sense, like omitting the final field separator because nothing comes after it.

    // edit: I have horribly misunderstood what a lexer is
    // it's very easy to put "meaning" into the tokens by accident. The lexer does not put meaning into the tokens at all. It's more just priming them for structure,
    // switching to Logos as a solution for the lexer reveals this, since you can't attach enum fields to Logos tokens. Type theory wins again..

    let mut lex = LTok::lexer(test);

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");
    
    assert_eq!(lex.next(), Some(Ok(LTok::Comment)));
    assert_eq!(lex.slice(), "%");

    assert_eq!(lex.next(), Some(Ok(LTok::Text)));
    assert_eq!(lex.slice(), "opening block separator");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Comment)));
    assert_eq!(lex.slice(), "%");

    assert_eq!(lex.next(), Some(Ok(LTok::Text)));
    assert_eq!(lex.slice(), "separator doesnt end until there is a non-comment");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Text)));
    assert_eq!(lex.slice(), "non comment");
    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Text)));
    assert_eq!(lex.slice(), "field separator before next non comment");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Text)));
    assert_eq!(lex.slice(), "block separator before bext object");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Comment)));
    assert_eq!(lex.slice(), "%");

    assert_eq!(lex.next(), Some(Ok(LTok::Text)));
    assert_eq!(lex.slice(), "block separator interrupted by comment");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");

    assert_eq!(lex.next(), Some(Ok(LTok::Text)));
    assert_eq!(lex.slice(), "last object");

    assert_eq!(lex.next(), Some(Ok(LTok::Separator)));
    assert_eq!(lex.slice(), "\n");
}
