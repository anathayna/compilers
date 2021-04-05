use std::env;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc != 2 {
        eprintln!("sintaxe: ehbasic <expressão>");
        process::exit(1);
    }
}
