extern crate bnf;
use bnf::Grammar;

fn main() {
    let input = r#"
        <expressao> ::= <termo> | <termo> "+" <termo>
        <termo> ::= "0" | "1"
    "#;
    let grammar: Grammar = input.parse().unwrap();
    let sentence = grammar.generate();
    match sentence {
        Ok(s) => println!("random sentence: {}", s),
        Err(e) => println!("something went wrong: {}!", e)
    }
}

// cargo check -- download lib bnf
// cargo run
