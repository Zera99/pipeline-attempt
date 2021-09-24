use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Default)]
pub struct WavHandler {
    // Riff Chunk
    pub riff_header: [u8; 4],
    pub riff_chunk_size: u32,
    pub wave_header: [u8; 4],
    // fmt subchunk
    pub fmt_header: [u8; 4],
    pub fmt_chunk_size: u32,
    pub audio_format: u16,
    pub channel_amount: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bit_rate: u16,
    // Data subchunk
    pub data_chunk: [u8; 4],
    pub data_size: u32,
}

impl WavHandler {
    pub fn new<R: Read>(audio_file: R) -> Result<WavHandler, &'static str> {
        let result: WavHandler = WavHandler::default(); // Unused for now. Will be populated by the file's details
        let mut reader: BufReader<R> = BufReader::new(audio_file);
        let mut line: [u8; 4] = [0; 4];
        match reader.read_exact(&mut line) {
            Ok(line) => line,
            Err(_) => return Err("Problem reading into the buffer."),
        }

        for value in line.iter() {
            println!("{}", *value as char);
        }

        Ok(result)
    }
}

// Step 1: Open the file
pub fn open_file(mut args: env::Args) -> Result<File, &'static str> {
    args.next(); // Skip the first one because it's the name of the program
    let possible_path = args.next();
    let path = match possible_path {
        Some(arg) => {
            if Path::new(&arg).exists() {
                arg
            } else {
                return Err("Filepath isn't valid");
            }
        }
        None => return Err("Didn't get any filepath!"),
    };

    let audio_file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(_) => {
            return Err("Something went bad when reading the file. {}");
        }
    };

    Ok(audio_file)
}

/*
    1- Open file - Done!
    2- Read File - In progress
    3- Edit File
    4- Save new file
    5- Open several files
    6- Process with Threads
    7- Save new files
    8- Combine all files
*/
