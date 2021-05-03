use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::fmt::Write as FmtWrite;

pub fn reader(file: Option<&str>) -> BufReader<Box<dyn Read>> {
    let read: Box<dyn Read> = if let Some(f) = file {
        Box::new(File::open(f).ok().unwrap())
    } else {
        Box::new(io::stdin())
    };
    return BufReader::new(read);
}

pub fn writer(file: Option<&str>) -> BufWriter<Box<dyn Write>> {
    let write: Box<dyn Write> = if let Some(f) = file {
        Box::new(File::create(f).ok().unwrap())
    } else {
        Box::new(io::stdout())
    };
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
