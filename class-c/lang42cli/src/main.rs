fn main() {
    env_logger::init();
    log::info!("aqui!");
    lang42lib::oi();
}

//RUST_LOG=info target/debug/lang42cli
//RUST_LOG=trace target/debug/lang42cli
