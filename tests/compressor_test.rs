// Unit tests for the compressor
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use bincode;

    #[test]
    fn test_compress_empty_input() {
        let input: Vec<u8> = Vec::new();
        let expected = vec![];
        let actual = compressor::compress(&input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_compress_single_char() {
        let input: Vec<u8> = vec![0x12];
        let expected = vec![0x12, 0x01];
        let actual = compressor::compress(&input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_compress_repeated_chars() {
        let mut rng = rand::thread_rng();
        let input: Vec<u8> = (0..10).map(|_| rng.gen()).collect();
        let expected = vec![0x12, 0x0a];
        let actual = compressor::compress(&input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decompress_empty_input() {
        let input: Vec<u8> = Vec::new();
        let expected = vec![];
        let actual = compressor::decompress(&input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decompress_single_char() {
        let input: Vec<u8> = vec![0x12, 0x01];
        let expected = vec![0x12];
        let actual = compressor::decompress(&input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decompress_repeated_chars() {
        let mut rng = rand::thread_rng();
        let input: Vec<u8> = (0..10).map(|_| rng.gen()).collect();
        let repeated = input.iter().fold((0, 1), |(count, last), &x| {
            if x == last {
                (count + 1, x)
            } else {
                (1, x)
            }
        });
        let expected = vec![repeated.1; repeated.0];
        let actual = compressor::decompress(&input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_compress_and_decompress() {
        let mut rng = rand::thread_rng();
        let input: Vec<u8> = (0..10).map(|_| rng.gen()).collect();
        let compressed = compressor::compress(&input).unwrap();
        let decompressed = compressor::decompress(&compressed).unwrap();
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_bincode_encoding() {
        let input: Vec<u8> = vec![0x12, 0x34];
        let encoded = bincode::serialize(&input).unwrap();
        let decoded: Vec<u8> = bincode::deserialize(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    #[should_panic]
    fn test_compress_invalid_input() {
        let input: Vec<u8> = vec![0x12];
        let _ = compressor::compress(&input).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decompress_invalid_input() {
        let input: Vec<u8> = vec![0x12, 0x01];
        let _ = compressor::decompress(&input).unwrap();
    }
}