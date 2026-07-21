// compressor.rs

use std::collections::HashMap;
use bincode::{serialize, deserialize};

/// Run-Length Encoding (RLE) compression algorithm implementation
pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
    // Check if input is empty
    if data.is_empty() {
        return Err("Input is empty".to_string());
    }

    let mut compressed = Vec::new();
    let mut char_count = HashMap::new();

    for byte in data {
        // Update character count
        *char_count.entry(*byte).or_insert(0) += 1;
    }

    // Sort characters by frequency
    let mut sorted_chars = char_count
        .into_iter()
        .map(|(c, count)| (count, c))
        .collect::<Vec<_>>();
    sorted_chars.sort_by(|a, b| b.0.cmp(&a.0));

    // Create run-length encoding
    let mut rle = Vec::new();
    for (_count, char_code) in sorted_chars {
        // Count occurrences of character in original data
        let mut count = 0;
        for byte in data {
            if *byte == char_code {
                count += 1;
            }
        }

        // Append to RLE
        rle.push(char_code);
        rle.push(count as u8);
    }

    // Serialize RLE into bytes
    compressed.extend(serialize(&rle).unwrap());

    Ok(compressed)
}

/// Decompresses run-length encoded data
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
    // Check if input is not empty
    if data.is_empty() {
        return Err("Input is empty".to_string());
    }

    let mut decompressed = Vec::new();
    let mut rle: Vec<u8> = Vec::new();

    // Deserialize RLE from bytes
    match deserialize(data) {
        Ok(rle_data) => rle.extend_from_slice(&rle_data),
        Err(err) => return Err(err.to_string()),
    }

    // Reconstruct original data
    for i in (0..rle.len()).step_by(2) {
        let char_code = rle[i];
        let count = rle[i + 1];

        // Repeat character count times
        for _ in 0..count {
            decompressed.push(char_code);
        }
    }

    Ok(decompressed)
}