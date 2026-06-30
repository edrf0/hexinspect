mod random_binary_file_generator;
mod file_analyzer;

use clap::Parser;
use crate::random_binary_file_generator::generate_random_binary_file;
use crate::file_analyzer::analyze_file;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long, default_value_t = false)]
    create: bool,
    #[arg(short, long, default_value_t = 1)]
    size_kb: usize,
}

fn main() {
    let args = Args::parse();
    if args.create {
        let final_size_kb: usize;
        match args.size_kb {
            size_kb @ 1..4_194_304 => {
                println!("Creating {} KB random binary file...", size_kb);
                final_size_kb = size_kb;
            },
            _ => {
                println!("Creating 1 KB random binary file...");
                final_size_kb = 1;
            },
        }
        match generate_random_binary_file(&args.path,final_size_kb) {
            Ok(()) => println!("Successfully generated random binary file at {}", args.path),
            Err(e) => println!("Failed to generate random binary file: {}", e),
        }
    } else {
        match analyze_file(&args.path) {
            Ok(()) => println!("Successfully analyzed binary file at {}", args.path),
            Err(e) => println!("Failed to analyze binary file: {}", e),
        }
    }
}
