use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

const DATA_CHUNK_SIZE: u8 = 128;

// Struct representing a canonical .WAV file
#[derive(Default, Debug, Clone)]
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
    pub audio_data: Vec<u8>,
}

#[derive(Default)]
struct ByteRepresentation {
    riff_header: [u8; 4],
    riff_chunk_size: [u8; 4],
    wave_header: [u8; 4],
    // fmt subchunk
    fmt_header: [u8; 4],
    fmt_chunk_size: [u8; 4],
    audio_format: [u8; 2],
    channel_amount: [u8; 2],
    sample_rate: [u8; 4],
    byte_rate: [u8; 4],
    block_align: [u8; 2],
    bit_rate: [u8; 2],
    // Data subchunk
    data_chunk: [u8; 4],
    data_size: [u8; 4],
    audio_data: Vec<u8>,
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

// Step 2.0: Read each byteline
fn get_next_byteline<R: Read>(reader: &mut BufReader<R>) -> Result<[u8; 4], &'static str> {
    let mut line: [u8; 4] = [0; 4];
    match reader.read_exact(&mut line) {
        Ok(line) => line,
        Err(_) => {
            return Err("Problem reading into the buffer in get_next_byteline");
        }
    }

    Ok(line)
}

// Step 2.2: Read data in bigger chunks
fn get_next_data_chunk<R: Read>(
    reader: &mut BufReader<R>,
    chunk_size: u8,
) -> Result<Vec<u8>, &'static str> {
    let mut line: Vec<u8> = vec![0; chunk_size.into()];
    match reader.read_exact(&mut line) {
        Ok(line) => line,
        Err(_) => {
            return Err("Problem reading into the buffer in get_next_data_chunk.");
        }
    }

    Ok(line)
}

impl WavHandler {
    // Step 2.3: Read the File
    pub fn new<R: Read>(audio_file: R) -> Result<WavHandler, &'static str> {
        let mut result: WavHandler = WavHandler::default(); // Unused for now. Will be populated by the file's details
        let mut reader: BufReader<R> = BufReader::new(audio_file);

        result.riff_header = get_next_byteline(&mut reader)?;
        result.riff_chunk_size = u32::from_le_bytes(get_next_byteline(&mut reader)?);
        result.wave_header = get_next_byteline(&mut reader)?;
        result.fmt_header = get_next_byteline(&mut reader)?;
        result.fmt_chunk_size = u32::from_le_bytes(get_next_byteline(&mut reader)?);

        let line = get_next_byteline(&mut reader)?;
        // Grab 2 slices from Line and convert them into u16
        result.audio_format = u16::from_le_bytes(<[u8; 2]>::try_from(&line[0..2]).unwrap());
        result.channel_amount = u16::from_le_bytes(<[u8; 2]>::try_from(&line[2..4]).unwrap());

        result.sample_rate = u32::from_le_bytes(get_next_byteline(&mut reader)?);
        result.byte_rate = u32::from_le_bytes(get_next_byteline(&mut reader)?);

        // Again, we grab 2 slices from the next line
        let line = get_next_byteline(&mut reader)?;
        result.block_align = u16::from_le_bytes(<[u8; 2]>::try_from(&line[0..2]).unwrap());
        result.bit_rate = u16::from_le_bytes(<[u8; 2]>::try_from(&line[2..4]).unwrap());

        result.data_chunk = get_next_byteline(&mut reader)?;
        result.data_size = u32::from_le_bytes(get_next_byteline(&mut reader)?);

        // Step 2.4: Read the Audio data
        if result.data_size % DATA_CHUNK_SIZE as u32 != 0 {
            println!("Data size cannot be divided by Chunk Size");
            return Err("Data size cannot be divided by Chunk Size");
        } else {
            let mut read_count = result.data_size / DATA_CHUNK_SIZE as u32;
            println!("Read amount: {}.", read_count);
            while read_count > 0 {
                let line = get_next_data_chunk(&mut reader, DATA_CHUNK_SIZE)?;
                for item in line {
                    result.audio_data.push(item);
                }
                read_count -= 1;
            }
        }

        Ok(result)
    }

    // Step 3.0: Edit the header
    pub fn change_channel_test(&mut self) {
        self.channel_amount = 1;
    }

    // Debug Function that I change depending on the current issue. Ignore it
    pub fn show(&self) {}

    pub fn write_new_file(&self) {
        let bytes_repr = ByteRepresentation::convert_to_bytes(self);
        ByteRepresentation::write_new_file(bytes_repr).unwrap();
    }
}

// Step 4.0: Convert WavHandler to it's proper byte representation to be written
impl ByteRepresentation {
    fn convert_to_bytes(wav_handler: &WavHandler) -> ByteRepresentation {
        let mut byte_repr = ByteRepresentation::default();
        byte_repr.riff_header = wav_handler.riff_header; // It's already a [u8], shouldn't need conversion
        byte_repr.riff_chunk_size = wav_handler.riff_chunk_size.to_le_bytes();
        byte_repr.wave_header = wav_handler.wave_header; // It's already a [u8], shouldn't need conversion
        byte_repr.fmt_header = wav_handler.fmt_header; // It's already a [u8], shouldn't need conversion
        byte_repr.fmt_chunk_size = wav_handler.fmt_chunk_size.to_le_bytes();
        byte_repr.audio_format = wav_handler.audio_format.to_le_bytes();
        byte_repr.channel_amount = wav_handler.channel_amount.to_le_bytes();
        byte_repr.sample_rate = wav_handler.sample_rate.to_le_bytes();
        byte_repr.byte_rate = wav_handler.byte_rate.to_le_bytes();
        byte_repr.block_align = wav_handler.block_align.to_le_bytes();
        byte_repr.bit_rate = wav_handler.bit_rate.to_le_bytes();
        byte_repr.data_chunk = wav_handler.data_chunk; // It's already a [u8], shouldn't need conversion
        byte_repr.data_size = wav_handler.data_size.to_le_bytes();
        byte_repr.audio_data = wav_handler.audio_data.clone(); // Vec of u8s. Don't think I need to convert it to [u8; 4] chunks. Gonna try without it

        byte_repr
    }

    // Step 4.1: Save a new file
    pub fn write_new_file(byte_repr: ByteRepresentation) -> io::Result<()> {
        let buffer = File::create("files/copied.wav")?;
        let mut buffer = BufWriter::new(buffer);

        buffer.write(&byte_repr.riff_header)?;
        buffer.write(&byte_repr.riff_chunk_size)?;
        buffer.write(&byte_repr.wave_header)?;
        buffer.write(&byte_repr.fmt_header)?;
        buffer.write(&byte_repr.fmt_chunk_size)?;
        buffer.write(&byte_repr.audio_format)?;
        buffer.write(&byte_repr.channel_amount)?;
        buffer.write(&byte_repr.sample_rate)?;
        buffer.write(&byte_repr.byte_rate)?;
        buffer.write(&byte_repr.block_align)?;
        buffer.write(&byte_repr.bit_rate)?;
        buffer.write(&byte_repr.data_chunk)?;
        buffer.write(&byte_repr.data_size)?;
        buffer.write(&byte_repr.audio_data)?;

        Ok(())
    }
}

/*
    1- Open file - Done!
    2- Read File - Done!
    3- Edit File - 25% Done! Only able to edit the header
    4- Save new file - Done!
    5- Open several files
    6- Process with Threads
    7- Save new files
    8- Combine all files
*/
