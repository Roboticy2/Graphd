use louder_rust::parser::*;
use pest::{Parser, iterators::Pairs};

#[test]
fn test_parser_compiles() {
    //No assertions. Just test that the parser loop works

    let test = "

        %opening block separator                        
        %separator doesnt end until there is a non-comment
        non comment
        field separator before next non comment

        key:value
        > choice
         harder key :   harder value?
        
        block separator before bext object
        %block separator interrupted by comment         
        last object
    ";

    let parse = LParser::parse(Rule::Program, test);

    println!("{:?}", parse);

    match parse {
        Err(_) => {
            assert!(false);
        }
        _ => {
            assert!(true);
        }
    };
}
