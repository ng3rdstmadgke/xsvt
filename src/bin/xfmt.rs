use xsvtools::utils;
use std::io::prelude::*;
use std::io::BufRead;
use clap::{App, Arg};
use std::process;
use std::fmt::Write as FmtWrite;
use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;

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
            Arg::with_name("INPUT_FILE")
            .value_name("FILE_NAME")
            .help("input file name")
        );
    // TODO: カラムのmaxwidth設定
    let matches = opts.get_matches();
    let delimiter: &str = matches
        .value_of("DELIMITER")
        .unwrap();
    let input_file = matches
        .value_of("INPUT_FILE");
    let reader = utils::reader(input_file);
    let mut writer = utils::writer(None);

    let mut buffer: Vec<Vec<String>> = vec![];
    let mut padding_util = None;
    for (ln, line) in reader.lines().enumerate() {
        let line: String = line.ok().unwrap();
        // TODO: 全部Stringにしてると遅い
        let cols: Vec<String> = line.split(delimiter).map(|e| e.to_string()).collect();
        if ln < 100 {
            buffer.push(cols);
        } else if ln == 100 {
            padding_util = Some(PaddingUtil::new(&buffer));
            if let Some(ref padding_util) = padding_util {
                for bcols in buffer.iter() {
                    if writer.write(padding_util.format(&bcols).as_bytes()).is_err() == true {
                        process::exit(0);
                    }
                }
                if writer.write(padding_util.format(&cols).as_bytes()).is_err() == true {
                    process::exit(0);
                }
            }
        } else {
            if let Some(ref padding_util) = padding_util {
                if writer.write(padding_util.format(&cols).as_bytes()).is_err() == true {
                    process::exit(0);
                }
            }
        }
    }
    if padding_util.is_none() {
        padding_util = Some(PaddingUtil::new(&buffer));
        if let Some(ref padding_util) = padding_util {
            for bcols in buffer.iter() {
                if writer.write(padding_util.format(&bcols).as_bytes()).is_err() == true {
                    process::exit(0);
                }
            }
        }
    }
}

#[derive(Debug)]
struct PaddingUtil {
    width_list: Vec<usize>,
    col_length: usize,
}

impl PaddingUtil {
    fn new(rows: &[Vec<String>]) -> Self {
        let col_length = if let Some(cols) = rows.get(0) { cols.len() } else { 0 };
        let mut width_list = vec![0; col_length];
        for i in 0..col_length {
            // カラムごとの値の最大幅を求める。上限は100
            let max_value_length = rows
                .iter()
                .map(|cols| {
                    let value: &str = if let Some(v) = cols.get(i) {v} else {&""};
                    let width =  UnicodeWidthStr::width(value) + 1;
                    if width > 100 {100} else {width}
                })
                .max()
                .unwrap();
            width_list[i] = max_value_length;
        }
        PaddingUtil { width_list, col_length }
    }

    fn format(&self, cols: &[String]) -> String {
        let mut ret = String::new();
        for (i, &width) in self.width_list.iter().enumerate() {
            let value: &str = if let Some(v) = cols.get(i) {v} else {&""};
            let value_len =  UnicodeWidthStr::width(value);
            if (i + 1) == self.col_length {
                // 最終カラムはすべて表示
                write!(ret, "| {}", value).unwrap();
            } else if value_len < width {
                // カラム幅よりも値が短い場合はパディング
                let pad_size = width - value_len;
                write!(ret, "| {}{} ", value, " ".repeat(pad_size)).unwrap();
            } else {
                // カラム幅よりも値が長い場合は値をカラム幅で切る
                write!(ret, "| ").unwrap();
                let mut cnt: usize = 0;
                for c in value.chars() {
                    let w = UnicodeWidthChar::width(c).unwrap_or(0);
                    if cnt + w < width {
                        write!(ret, "{}", c).unwrap();
                    } else {
                        write!(ret, "{}", " ".repeat(width - cnt)).unwrap();
                        break;
                    }
                    cnt = cnt + w;
                }
                write!(ret, " ").unwrap();
            }
        }
        write!(ret, "\n").unwrap();
        return ret;
    }
}
