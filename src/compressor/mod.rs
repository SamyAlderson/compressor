// Module declaration for compressor.rs
mod compressor {
    pub use self::compress;
    pub use self::decompress;

    use crate::utils::run_length_encoding;
    use bincode::serialize;

    /// Compresses input data using run-length encoding (RLE).
    ///
    /// This function takes a `Vec<u8>` as input, compresses it using RLE, and returns a `Vec<u8>` representing the compressed data.
    ///
    /// # Errors
    ///
    /// Returns a `std::io::Error` if there's an issue serializing the compressed data.
    pub fn compress(data: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        let encoded = run_length_encoding(data);
        serialize(&encoded).map_err(Into::into)
    }

    /// Decompresses input data using run-length encoding (RLE).
    ///
    /// This function takes a `Vec<u8>` as input, decompresses it using RLE, and returns a `Vec<u8>` representing the decompressed data.
    ///
    /// # Errors
    ///
    /// Returns a `std::io::Error` if there's an issue deserializing the compressed data.
    pub fn decompress(data: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        let decoded: Vec<(u8, usize)> = bincode::deserialize(&data).map_err(Into::into)?;
        let decompressed = decoded
            .into_iter()
            .map(|(char, count)| char.repeat(count))
            .flatten()
            .collect();
        Ok(decompressed)
    }
}