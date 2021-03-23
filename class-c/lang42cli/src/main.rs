fn main() {
    init();
    std::env::args();
    log::trace!("init main func");
    //log::info!("aqui!");
    lang42lib::version();
    log::trace!("end main func");
}

fn init() {
    env_logger::init();
}

//RUST_LOG=info target/debug/lang42cli
//RUST_LOG=trace target/debug/lang42cli
//RUST_LOG=trace cargo run
//RUST_LOG=info,lang42lib=trace,lang42cli=trace cargo run
