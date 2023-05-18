use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize, // 这个是必须有的
    bytes: Option<usize>, // 这个可有可无， 与lines对立
}

pub fn run(config: Config) -> MyResult<()> {
    for (_, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if let Some(bytes) = config.bytes {
                    let mut buf = vec![0; bytes];
                    let _ = file.read_exact(&mut buf)?;
                    println!("{}", String::from_utf8_lossy(&buf));
                    // let mut handle = file.take(bytes as u64);
                    // let mut buffer = vec![0; bytes];
                    // let bytes_read = handle.read(&mut buffer)?;
                    // println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    for (_, line) in file.lines().enumerate().take(config.lines) {
                        println!("{}", line.unwrap());
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("xiaozhu")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input files")
                .multiple(true),
        )
        .get_matches();

    let lines = matches
        .value_of("lines") // 获取lines的值 返回的是一个Option<&str>
        .map(parse_positive_int) // 映射成功的话往下走，不成功map_err
        .transpose() // 将内部的Result和外部的Option互换
        .map_err(|e| format!("illegal byte count: {}, count should be digital", e))?;
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count: {}, count should be digital", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
