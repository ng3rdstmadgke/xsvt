use xsvtools::utils;
use std::io::prelude::*;
use std::io::BufRead;
use clap::{App, Arg};


fn main() {
    let opts = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("DELIMITER")
            .short("d")
            .long("delimiter")
            .value_name("DELIMITER")
            .help("field delimiter")
            .takes_value(true)
            .default_value("\t")
        )
        .arg(
            Arg::with_name("INDEXES")
            .short("i")
            .long("indexes")
            .value_name("FIELD_INDEX_1,FIELD_INDEX_2,...")
            .help("field index.(1, 2, 3, ...)")
            .takes_value(true)
            .value_delimiter(",")
            .default_value("")
            //.required(true)
        )
        .arg(
            Arg::with_name("FIELDS")
            .short("f")
            .long("fields")
            .value_name("FIELD_NO_1,FIELD_NO_2,...")
            .help("field name.(col1, col2, col3, ...)")
            .takes_value(true)
            .value_delimiter(",")
            .default_value("")
            //.required(true)
        )
        .arg(
            Arg::with_name("INPUT_FILE")
            .value_name("FILE_NAME")
            .help("input file name")
        );

    let matches = opts.get_matches();
    let delimiter: &str = matches
        .value_of("DELIMITER")
        .unwrap();

    let indexes = matches
        .values_of("INDEXES")
        .unwrap()
        .map(|e| e.trim())
        .filter(|e| e.len() != 0)
        .map(|e| e.parse::<usize>().ok().unwrap())
        .collect::<Vec<usize>>();

    let fields = matches
        .values_of("FIELDS")
        .unwrap()
        .filter(|e| e.len() != 0)
        .collect::<Vec<&str>>();

    let input_file = matches
        .value_of("INPUT_FILE");

    let reader = utils::reader(input_file);
    let mut writer = utils::writer(None);
    let mut field_map = None;
    for (ln, line) in reader.lines().enumerate() {
        let line: String = line.ok().unwrap();
        let cols: Vec<&str> = line.split(delimiter).collect();
        if ln == 0 {
            field_map = Some(FieldMap::new(&indexes, &fields, &cols));
            println!("{:?}", field_map);
        }
        if let Some(ref field_map) = field_map {
            for (i, &idx) in field_map.map.iter().enumerate() {
                if i > 0 {
                    writer.write(delimiter.as_bytes()).unwrap();
                }
                if let Some(value) = cols.get(idx) {
                    writer.write(value.as_bytes()).unwrap();
                }
            }
            writer.write("\n".as_bytes()).unwrap();
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct FieldMap {
    indexes: Vec<usize>,
    fields: Vec<String>,
    header: Vec<String>,
    map: Vec<usize>
}

impl FieldMap {
    fn new(indexes: &[usize], fields: &[&str], header: &[&str]) -> FieldMap {
        let mut map: Vec<usize> = vec![];
        for &index in indexes.iter() {
            if index > 0 && index < header.len() {
                map.push(index - 1);
            } else {
                panic!("index {} is not found.", index);
            }
        }
        for (i, field) in fields.iter().enumerate() {
            if header.contains(field) {
                map.push(i);
            } else {
                panic!("field {} is not found.", i);
            }
        }
        FieldMap {
            indexes: indexes.iter().map(|e| *e).collect(),
            fields : fields.iter().map(|e| e.to_string()).collect(),
            header : header.iter().map(|e| e.to_string()).collect(),
            map
        }
    }
}
