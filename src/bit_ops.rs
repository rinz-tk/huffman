use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write, Error};

pub struct BitReader {
    buf: [u8; 1],
    left: u8,
    reader: BufReader<File>
}

impl BitReader {
    pub fn new(reader: BufReader<File>) -> BitReader {
        BitReader {
            buf: [0],
            left: 0,
            reader
        }
    }

    pub fn get_mut_ref(&mut self) -> &mut BufReader<File> {
        &mut self.reader
    }

    pub fn read_bit(&mut self) -> Result<u8, Error> {
        if self.left == 0 {
            self.reader.read(&mut self.buf)?;
            self.left = 8;
        }

        let d = (self.buf[0] >> (self.left - 1)) & 1;
        self.left -= 1;

        Ok(d)
    }
}

pub struct BitWriter {
    buf: u8,
    left: u8,
    writer: BufWriter<File>
}

impl BitWriter {
    pub fn new(writer: BufWriter<File>) -> BitWriter {
        BitWriter {
            buf: 0,
            left: 8,
            writer
        }
    }

    pub fn get_mut_ref(&mut self) -> &mut BufWriter<File> {
        &mut self.writer
    }

    pub fn write_bit(&mut self, bit: u8) -> Result<(), Error> {
        self.buf = (self.buf << 1) + bit;

        self.left -= 1;
        if self.left == 0 {
            self.writer.write_all(&[self.buf])?;

            self.buf = 0;
            self.left = 8;
        }

        Ok(())
    }

    pub fn flush_buf(&mut self) -> Result<(), Error> {
        if self.left != 8 {
            self.buf <<= self.left;

            self.writer.write_all(&[self.buf])?;
        }

        Ok(())
    }
}
