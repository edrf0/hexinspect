use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::{Path};
use rand::{rng, RngExt};

pub fn generate_random_binary_file(path: &str,size_kb: usize) -> Result<(),io::Error> {
    let mut random_generator = rng();
    let mut vector: Vec<u8> = Vec::with_capacity(size_kb * 1024);
    for _ in 0..vector.capacity() {
        vector.push(random_generator.random());
    }
    let file_handle = File::create(Path::new(path))?;
    let mut writer = BufWriter::new(file_handle);
    writer.write_all(&vector)?;
    writer.flush()?;
    Ok(())
}