mod node;
mod bit_ops;

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::io::{BufReader, BufWriter, Read, Write, Seek, Error, ErrorKind};
use std::env;

use node::Node;
use bit_ops::{BitReader, BitWriter};

fn prefix_codes_r(tree: &Box<Node>, codes: &mut HashMap<u8, Vec<u8>>, code: Vec<u8>) {
    match tree.c {
        Some(ch) => {
            // println!("{}: {} ({})", ch as char, code.iter().fold(String::new(), |s, e| s + &e.to_string()), tree.f);
            codes.insert(ch, code);
        },
        None => {
            let mut left_code = code.clone();
            left_code.push(0);
            prefix_codes_r(tree.left.as_ref().unwrap(), codes, left_code);

            let mut right_code = code.clone();
            right_code.push(1);
            prefix_codes_r(tree.right.as_ref().unwrap(), codes, right_code);
        }
    }
}

fn prefix_codes(tree: &Box<Node>) -> HashMap<u8, Vec<u8>> {
    let mut codes: HashMap<u8, Vec<u8>> = HashMap::new();

    prefix_codes_r(tree, &mut codes, Vec::new());

    codes
}

fn write_tree(writer: &mut BitWriter, tree: &Box<Node>) -> Result<(), Error> {
    match tree.c {
        Some(ch) => {
            writer.write_bit(1)?;

            for i in 0..8 {
                writer.write_bit((ch >> (7 - i)) & 1)?;
            }
        },

        None => {
            writer.write_bit(0)?;

            write_tree(writer, tree.left.as_ref().unwrap())?;
            write_tree(writer, tree.right.as_ref().unwrap())?;
        }
    }

    Ok(())
}

fn write_encoded_file(mut filename: PathBuf, input: BufReader<File>, codes: HashMap<u8, Vec<u8>>, tree: Box<Node>) -> Result<(), Error> {
    let ext = filename.extension();

    let mut ext_hf = OsString::new();
    if let Some(e) = ext {
        ext_hf = e.to_os_string();
        ext_hf.push(".");
    }

    ext_hf.push("hf");
    filename.set_extension(ext_hf);
    let filename = Path::new(filename.file_name().unwrap());

    let mut writer = BitWriter::new(BufWriter::new(File::create(filename)?));

    let file_size: u64 = input.get_ref().metadata()?.len();
    writer.get_mut_ref().write_all(&file_size.to_be_bytes())?;

    write_tree(&mut writer, &tree)?;

    input.get_ref().rewind()?;
    for b in input.bytes() {
        let b = b?;
        let code = codes.get(&b).unwrap();

        for bit in code {
            writer.write_bit(*bit)?;
        }
    }

    writer.flush_buf()?;
    writer.get_mut_ref().flush()?;

    Ok(())
}

fn encode(filename: PathBuf) -> Result<(), Error> {
    let mut input = BufReader::new(File::open(filename.clone())?);

    if input.get_ref().metadata()?.len() == 0 {
        return Err(Error::new(ErrorKind::NotFound, "file to be compressed is empty"));
    }

    let mut freq: HashMap<u8, u32> = HashMap::new();

    let mut buf: [u8; 1] = [0];
    while let Ok(1) = input.read(&mut buf) {
        *freq.entry(buf[0]).or_insert(0) += 1;
    }

    let mut heap: BinaryHeap<Box<Node>> = BinaryHeap::new();
    for (c, f) in freq {
        heap.push(Box::new(Node {
            c: Some(c),
            f,
            left: None,
            right: None
        }));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let f = left.f + right.f;
        let c = None;

        let z = Box::new(Node {
            c,
            f,
            left: Some(left),
            right: Some(right)
        });

        heap.push(z);
    }

    let tree = heap.pop().unwrap();
    let codes = prefix_codes(&tree);

    write_encoded_file(filename, input, codes, tree)?;

    Ok(())
}

fn read_tree(reader: &mut BitReader, tree: &mut Box<Node>) -> Result<(), Error> {
    let d = reader.read_bit()?;

    if d == 0 {
        tree.left = Some(Box::new(Node::new()));
        tree.right = Some(Box::new(Node::new()));

        read_tree(reader, tree.left.as_mut().unwrap())?;
        read_tree(reader, tree.right.as_mut().unwrap())?;
    } else {
        let mut b: u8 = 0;

        for _ in 0..8 {
            let next = reader.read_bit()?;
            b = (b << 1) + next;
        }

        tree.c = Some(b);
    }

    Ok(())
}

fn decode(mut filename: PathBuf) -> Result<(), Error> {
    let mut reader = BitReader::new(BufReader::new(File::open(filename.clone())?)); 

    let ext = filename.extension().unwrap();
    if ext != "hf" { return Err(Error::new(ErrorKind::InvalidInput, "File must have a '.hf' extension to be decompressed")); }

    filename.set_extension("");
    let filename = Path::new(filename.file_name().unwrap());
    let mut writer = BufWriter::new(File::create(filename)?);

    let mut tree = Box::new(Node::new());

    let mut size_buf: [u8; 8] = [0; 8];
    reader.get_mut_ref().read_exact(&mut size_buf)?;
    let mut to_write = u64::from_be_bytes(size_buf);

    read_tree(&mut reader, &mut tree)?;

    let mut nav = &tree;
    while to_write > 0 {
        let d = reader.read_bit()?;

        if d == 0 { nav = nav.left.as_ref().unwrap(); }
        else { nav = nav.right.as_ref().unwrap(); }

        if let Some(c) = nav.c {
            writer.write_all(&[c])?;
            nav = &tree;

            to_write -= 1;
        }
    }

    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(Error::new(ErrorKind::InvalidInput, "Usage: huffman [-c | -x] filename"));
    }

    match args[1].as_str() {
        "-c" => { encode(PathBuf::from(&args[2]))? },

        "-x" => { decode(PathBuf::from(&args[2]))? },

        _ => {
            return Err(Error::new(ErrorKind::InvalidInput, "Usage: huffman [-c | -x] filename"));
        }
    }

    Ok(())
}
