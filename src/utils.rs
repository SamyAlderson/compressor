// src/utils.rs

/// Calculate the run length encoding for a given sequence of bytes.
///
/// This function compresses a sequence of bytes by counting consecutive repeated
/// bytes and replacing them with a single byte and a count of occurrences.
///
/// # Panics
///
/// This function will panic if the input sequence is empty.
pub fn rle_compress(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        panic!("Cannot compress an empty sequence");
    }

    let mut compressed = Vec::new();
    let mut current_byte = input[0];
    let mut byte_count = 1;

    for byte in input.iter().skip(1) {
        if *byte == current_byte {
            byte_count += 1;
        } else {
            compressed.push(current_byte);
            compressed.extend_from_slice(&byte_count.to_le_bytes());
            current_byte = *byte;
            byte_count = 1;
        }
    }

    compressed.push(current_byte);
    compressed.extend_from_slice(&byte_count.to_le_bytes());
    compressed
}

/// Decompress a run length encoded sequence of bytes.
///
/// This function takes a sequence of bytes where each byte is followed by a
/// count of occurrences and returns the original sequence of bytes.
///
/// # Panics
///
/// This function will panic if the input sequence has an odd length or if any
/// byte is not followed by a valid count.
pub fn rle_decompress(input: &[u8]) -> Vec<u8> {
    if input.len() % 2 != 0 {
        panic!("Invalid decompressed sequence");
    }

    let mut decompressed = Vec::new();
    let mut current_byte = input[0];
    let mut count = u16::from_le_bytes([input[1], input[2]]);

    for i in (2..input.len()).step_by(2) {
        let byte = input[i];
        if byte != current_byte {
            decompressed.push(byte);
            current_byte = byte;
            count = u16::from_le_bytes([input[i + 1], input[i + 2]]);
        } else if count == 0 {
            panic!("Count cannot be zero");
        } else {
            count -= 1;
        }
    }

    decompressed.push(current_byte);
    decompressed
}