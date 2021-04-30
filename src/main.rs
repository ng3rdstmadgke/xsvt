use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() {
    let mut writer = writer();
    for (_i, line) in reader().lines().enumerate() {
        let line = line.ok().unwrap();
        writer.write(line.as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}

#[allow(dead_code)]
fn reader() -> BufReader<Box<dyn Read>> {
    let read: Box<dyn Read> = Box::new(io::stdin());
    return BufReader::new(read);
}

#[allow(dead_code)]
fn writer() -> BufWriter<Box<dyn Write>> {
    let write: Box<dyn Write> = Box::new(io::stdout());
    return BufWriter::new(write);
}
