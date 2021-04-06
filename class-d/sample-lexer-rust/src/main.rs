use std::env;
use std::process;

mod lexer;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc != 2 {
        eprintln!("sintaxe: ehbasic <expressão>");
        process::exit(1);
    }

    let simbolos = lexer::lexer(&argv[1]);
    println!("{:?}", simbolos)
}

// cargo run -- 9
// cargo run 6+6
