use std::env;
use std::fs::File;
use std::process;

mod lib;
use lib::WavHandler;

fn main() {
    let file: File = lib::open_file(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem opening the file: {}", err);
        process::exit(1);
    });

    let wav_handler: WavHandler = WavHandler::new(file).unwrap_or_else(|err| {
        eprintln!("Problem processing the arguments: {}", err);
        process::exit(1);
    });

    println!("");
}
