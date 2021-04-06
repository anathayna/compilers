use std::env;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc != 2 {
        eprintln!("sintaxe: ehbasic <expressão>");
        process::exit(1);
    }

    let simbolos = lexer(&argv[1]);
    println!("{:?}", simbolos)
}

#[derive(Debug)]
enum ItemLexico {
    Numero(u64)
}

fn lexer(exp: &String) -> Vec<ItemLexico> {
    //unimplemented!();
    let mut r = Vec::new();
    let numero = exp.parse().unwrap();
    r.push(ItemLexico::Numero(numero));
    r
}

// cargo run -- 9
