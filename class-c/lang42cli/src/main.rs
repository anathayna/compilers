mod args;

use args::Argumentos as Args;

fn main() {
    init();
    //std::env::args();
    //log::trace!("init main func");
    //log::info!("aqui!");
    //lang42lib::version();

    let comando = Args::Inline;
    println!("{:?}", comando);

    //log::trace!("end main func");
}

fn init() {
    dotenv::dotenv().ok();
    env_logger::init();
}

//RUST_LOG=info target/debug/lang42cli
//RUST_LOG=trace target/debug/lang42cli
//RUST_LOG=trace cargo run
//RUST_LOG=info,lang42lib=trace,lang42cli=trace cargo run
//cargo build & cargo run