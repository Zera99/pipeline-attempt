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

    wav_handler.show();
    let mut new_handler: WavHandler = wav_handler.clone();
    new_handler.change_channel_test();
    new_handler.show();
    new_handler.write_new_file();

    println!("");
}
