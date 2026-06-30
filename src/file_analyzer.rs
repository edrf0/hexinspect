use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn analyze_file(path: &str) -> Result<(),io::Error> {
    let file_handle = File::open(Path::new(path))?;
    let mut reader = BufReader::new(file_handle);
    let mut buffer = [0u8;16];
    let mut offset = 0x0_usize;
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        print!("{:#018X} | ",offset);
        let current_chunk = &buffer[0..bytes_read];
        for byte in current_chunk {
            print!("{:02X} ", *byte);
        }
        print!("| ");
        for byte in current_chunk {
            match *byte {
                0x20..=0x7E => print!("{} ", *byte as char),
                _ => print!(". "),
            }
        }
        println!();
        offset += 0x10_usize;
    }
    println!();
    Ok(())
}