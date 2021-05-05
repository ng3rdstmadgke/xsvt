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
            Arg::with_name("MAX_WIDTH")
            .short("w")
            .long("max-width")
            .value_name("MAX_WIDTH")
            .help("max width by column")
            .takes_value(true)
            .default_value("100")
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
    let max_width: usize = matches // TODO: 数値パースのエラーハンドリング
        .value_of("MAX_WIDTH")
        .map(|v| v.parse::<usize>().ok().unwrap())
        .unwrap();
    let input_file = matches
        .value_of("INPUT_FILE");
    let reader = utils::reader(input_file);
    let mut writer = utils::writer(None);
    let mut line_buffer: Vec<String> = vec![];
    let mut formatter = None;
    for (ln, line) in reader.lines().enumerate() {
        let line: String = line.ok().unwrap();
        if ln < 100 {
            line_buffer.push(line);
        } else if ln == 100 {
            let rows = line_buffer
                .iter()
                .map(|l| l.split(delimiter).collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            formatter = Some(XsvFormatter::new(&rows, max_width));
            if let Some(ref formatter) = formatter {
                for cols in rows.iter() {
                    write_line(&mut writer, formatter.format(&cols));
                }
                let cols: Vec<&str> = line.split(delimiter).collect();
                write_line(&mut writer, formatter.format(&cols));
            }
        } else {
            let cols: Vec<&str> = line.split(delimiter).collect();
            if let Some(ref formatter) = formatter {
                write_line(&mut writer, formatter.format(&cols));
            }
        }
    }
    // 入力が100行以下だったときにline_bufferを出力する
    if formatter.is_none() {
        let rows = line_buffer
            .iter()
            .map(|l| l.split(delimiter).collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        formatter = Some(XsvFormatter::new(&rows, max_width));
        if let Some(ref formatter) = formatter {
            for cols in rows.iter() {
                write_line(&mut writer, formatter.format(&cols));
            }
        }
    }
}

fn write_line(writer: &mut dyn Write, line: String) {
    if writer.write(line.as_bytes()).is_err() == true {
        process::exit(0);
    }
    if writer.write("\n".as_bytes()).is_err() == true {
        process::exit(0);
    }

}

#[derive(Debug)]
struct XsvFormatter {
    width_list: Vec<usize>, // 各カラムの表示幅リスト
    col_length: usize,      // ヘッダ行のカラム数
    max_width: usize,       // カラムの最大幅
}

impl XsvFormatter {
    fn new(rows: &[Vec<&str>], max_width: usize) -> Self {
        let col_length = if let Some(cols) = rows.get(0) { cols.len() } else { 0 };
        let mut width_list = vec![0; col_length];
        for i in 0..col_length {
            // カラムごとの値の最大幅を求める。上限は100
            let max_value_length = rows
                .iter()
                .map(|cols| {
                    let value: &str = cols.get(i).unwrap_or(&"");
                    let width =  UnicodeWidthStr::width(value) + 1;
                    if width > max_width {max_width} else {width}
                })
                .max()
                .unwrap();
            width_list[i] = max_value_length;
        }
        XsvFormatter { width_list, col_length, max_width }
    }

    fn format(&self, cols: &[&str]) -> String {
        let mut ret = String::new();
        for (i, &width) in self.width_list.iter().enumerate() {
            let value: &str = cols.get(i).unwrap_or(&"");
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
        return ret;
    }
}
