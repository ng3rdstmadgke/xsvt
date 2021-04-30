use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
//use std::io::Write as IoWrite;
//use std::io::Read as IoRead;
use std::fmt::Write as FmtWrite;

pub fn reader() -> BufReader<Box<dyn Read>> {
    let read: Box<dyn Read> = Box::new(io::stdin());
    return BufReader::new(read);
}

pub fn writer() -> BufWriter<Box<dyn Write>> {
    let write: Box<dyn Write> = Box::new(io::stdout());
    return BufWriter::new(write);
}

pub fn join<T: std::fmt::Display>(delimiter: char, arr: &[T]) -> String {
    let mut text = String::new();
    for (i, e) in arr.iter().enumerate() {
        if i > 0 {
            text.push(delimiter);
        }
        write!(text, "{}", e).unwrap();
    }
    return text;
}
