// Library functions for the compressor library

// Import the necessary libraries
use bincode::{serialize, deserialize};
use rand::Rng;

// Function to compress a byte string using run-length encoding
pub fn rle_compress(input: &[u8]) -> Result<Vec<u8>, String> {
    // Initialize an empty vector to store the compressed data
    let mut compressed = Vec::new();

    // Initialize variables to keep track of the current run
    let mut current_char = input[0];
    let mut run_length = 1;

    // Iterate over the input data
    for (i, &byte) in input.iter().skip(1).enumerate() {
        // If the current byte is the same as the previous one, increment the run length
        if byte == current_char {
            run_length += 1;
        } else {
            // Otherwise, append the current run to the compressed data
            compressed.extend(&[current_char, run_length as u8]);

            // Update the current character and reset the run length
            current_char = byte;
            run_length = 1;
        }
    }

    // Append the last run to the compressed data
    compressed.extend(&[current_char, run_length as u8]);

    // Return the compressed data
    Ok(compressed)
}

// Function to decompress a byte string using run-length encoding
pub fn rle_decompress(input: &[u8]) -> Result<Vec<u8>, String> {
    // Initialize an empty vector to store the decompressed data
    let mut decompressed = Vec::new();

    // Iterate over the input data in steps of 2
    for i in (0..input.len()).step_by(2) {
        // Extract the current character and run length from the input data
        let char_code = input[i];
        let run_length = input[i + 1] as usize;

        // Append the current character to the decompressed data the specified number of times
        decompressed.extend(std::iter::repeat(char_code).take(run_length));
    }

    // Return the decompressed data
    Ok(decompressed)
}

// Function to generate a random byte string of a specified length
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    // Use the rand library to generate a random byte string
    let mut rng = rand::thread_rng();
    let mut bytes = Vec::with_capacity(length);
    for _ in 0..length {
        bytes.push(rng.gen());
    }
    bytes
}