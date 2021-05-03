use std::env;
use crate::lexer::avaliar;

mod lexer;

fn main() {
    let codigo = env::args().nth(1).expect("expressão não fornecida");
    if let Ok(resultado) = avaliar(&codigo) {
        println!("expressão {} = {}", codigo, resultado);
    } else {
        eprintln!("não foi possível avaliar a expressão");
    }
}

// cargo run 6+6
