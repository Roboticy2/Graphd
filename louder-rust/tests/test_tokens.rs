use louder_rust::lexer::LTok;

#[test]
fn test_import() {
    let a = LTok::BlockSeparator;
    assert_eq!(a, LTok::BlockSeparator);
}
