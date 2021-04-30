use xsvtools::utils;
use std::io::prelude::*;
use std::io::BufRead;

fn main() {
    let writer = utils::writer();
    let reader = utils::reader();
    let fields = [0, 1, 2];
    let delimiter = ",";
    cut(Box::new(reader), Box::new(writer), &fields, delimiter);
}


fn cut(reader: Box<dyn BufRead>, mut writer: Box<dyn Write>, fields: &[usize], delimiter: &str) {
    for line in reader.lines() {
        let line: String = line.ok().unwrap();
        let cols: Vec<&str> = line.split(delimiter).collect();
        for (i, &field) in fields.iter().enumerate() {
            if i > 0 {
                writer.write(delimiter.as_bytes()).unwrap();
            }
            writer.write(cols[field].as_bytes()).unwrap();
        }
        writer.write("\n".as_bytes()).unwrap();
    }
}


