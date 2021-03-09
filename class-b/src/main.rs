use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is: {}", secret_number);
    println!("please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("you guessed: {}", guess);

        let guess: u32 = guess.trim().parse::<u32>().expect("please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small! try again:"),
            Ordering::Greater => println!("too big! try again:"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}


// cargo new class-b --bin/--lib
// cargo run
// rustup component/toolchain list | less
// cargo build --release
// install dependences & libs: cargo build 