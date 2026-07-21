# Compressor
A simple RLE compressor in Rust

This project implements a basic compression algorithm using run-length encoding (RLE) in Rust. It's a proof-of-concept for exploring compression techniques and improving code readability.

## Installation and Usage
```bash
cargo add compressor
cargo run -- < input.txt
```
This will compress the input file using RLE and output the compressed data to stdout.

## Building from Source
```bash
git clone https://github.com/samyalderson/compressor.git
cd compressor
cargo build
```
This will build the compressor binary from source.

## Running Tests
```bash
cargo test
```
This will run the test suite and verify the compressor's correctness.

## Project Structure
* `src/main.rs`: The entry point of the project, responsible for compressing input data.
* `src/compressor.rs`: The RLE compression algorithm implementation.
* `src/decoder.rs`: The RLE decompression algorithm implementation.
* `tests/compression.rs`: Test cases for the compression algorithm.
* `tests/decompression.rs`: Test cases for the decompression algorithm.
* `Cargo.toml`: The project configuration file.

## License
Copyright (c) 2026 SamyAlderson

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.