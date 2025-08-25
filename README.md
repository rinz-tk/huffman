# Huffman

This project is an implementation of the Huffman algorithm in Rust. It provides a simple command-line interface for compressing and decompressing files using lossless Huffman encoding.

## Usage

huffman [-c | -x] filename

### Options
- `-c` : Compress the input file losslessly and produce an output file named `filename.hf`.  
- `-x` : Decompress a `.hf` file and restore the original file.

## Example

# Compress
huffman -c input.txt

# Decompress
huffman -x input.txt.hf


This will generate `input.txt.hf` when compressing and restore `input.txt` when decompressing.
