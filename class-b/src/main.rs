use std::io;
use rand::Rng;

fn main() {
    println!("*** jogo da advinhação ***");
    println!("  ~ informe seu palpite ~");
    println!("--------------------------");

    let _secret_number = rand::thread_rng().gen_range(1,101);
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("falha na leitura!");

    println!("seu palpite: {}", guess);
}


// cargo new class-b --bin/--lib
// cargo run
// rustup component/toolchain list | less
// cargo build --release