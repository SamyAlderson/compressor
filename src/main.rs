// Main entry point for the compressor application
fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Error: Please provide two file paths as arguments.");
        return;
    }
    let input_path = &args[1];
    let output_path = &args[2];

    // Check if input and output files exist
    std::fs::File::open(input_path).map_err(|e| {
        eprintln!("Error opening input file {}: {}", input_path, e);
        std::process::exit(1);
    });
    std::fs::File::open(output_path).map_err(|e| {
        eprintln!("Error opening output file {}: {}", output_path, e);
        std::process::exit(1);
    });

    // Run compression
    let compressor = Compressor::new();
    let compressed_data = compressor.compress(input_path);
    match compressed_data {
        Ok(compressed) => {
            // Write compressed data to file
            let mut output_file = std::fs::File::create(output_path).unwrap();
            bincode::serialize_into(&mut output_file, &compressed).unwrap();
            println!("Compression complete.");
        }
        Err(e) => {
            eprintln!("Error compressing {}: {}", input_path, e);
            std::process::exit(1);
        }
    }
}

// Import dependencies
use std::env;
use std::fs;
use std::io;
use bincode;
use rand;

// Import local modules
mod compressor;
mod utils;

// Import compressor module
use compressor::Compressor;
```

```rust
// Module declaration for the compressor
pub mod compressor {
    use super::utils;

    // Compression algorithm implementation
    pub struct Compressor {
        // Random number generator
        rand: rand::Rng,
    }

    impl Compressor {
        // Create a new compressor instance
        pub fn new() -> Self {
            let rand = rand::thread_rng();
            Compressor { rand }
        }

        // Compress data from a file
        pub fn compress(&self, input_path: &str) -> Result<Vec<u8>, String> {
            // Read input file
            let input_file = std::fs::File::open(input_path).map_err(|e| {
                format!("Error opening input file {}: {}", input_path, e)
            })?;

            // Create a byte buffer to store compressed data
            let mut compressed = Vec::new();

            // Iterate over input file and compress data
            let mut input = std::io::BufReader::new(input_file);
            loop {
                let mut byte = [0u8];
                match input.read(&mut byte) {
                    Ok(0) => break,
                    Ok(_) => {
                        // Run-length encoding
                        let mut count = 1;
                        while input.read(&mut byte).unwrap() == Ok(0) {
                            count += 1;
                        }
                        compressed.extend(vec![byte[0], count as u8]);
                    }
                    Err(e) => return Err(format!("Error reading input file {}: {}", input_path, e)),
                }
            }

            // Return compressed data
            Ok(compressed)
        }
    }
}
```

```rust
// Module declaration for utilities
pub mod utils {
    // Utility function to create a random number generator
    pub fn rand() -> rand::Rng {
        rand::thread_rng()
    }
}