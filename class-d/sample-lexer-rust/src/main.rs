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

fn lexer(exp: &String) -> Result<Vec<ItemLexico>, String> {
    //unimplemented!();
    let mut r = Vec::new();
    let numero_result = exp.parse();
    match numero_result {
        Ok(numero) => {
            r.push(ItemLexico::Numero(numero));
            return Ok(r);
        }
        Err(_) => {
            return Err(format!("valor inesperado: {}", exp));
        }
    }
}

// cargo run -- 9
