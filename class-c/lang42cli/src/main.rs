fn main() {
    //env_logger::init();
    log::trace!("init version func");
    //log::info!("aqui!");
    lang42lib::version();
    log::trace!("end version func");
}

//RUST_LOG=info target/debug/lang42cli
//RUST_LOG=trace target/debug/lang42cli
