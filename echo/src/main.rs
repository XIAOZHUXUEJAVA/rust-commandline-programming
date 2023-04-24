use clap::{App, Arg};
use std::fs::File;
use std::io::Write;
fn main() {
    let matches = App::new("echo")
        .version("0.1.0") // cargo run -- --version or cargo run -- -V
        .author("xiaozhu xiaozhuzhulzq@163.com")
        .about("Rust echo")
        .arg(Arg::with_name("string").multiple(true).required(true))
        .arg(
            Arg::with_name("file")
                .short(">")
                .long("out_put")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    let text = matches
        .values_of("string")
        .unwrap()
        .collect::<Vec<_>>()
        .join(" ");
    // let text = matches.value_of_lossy("string").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    // 参数文件不存在时，新建文件并新添一行 ， 存在， 新添一行 而不是 覆盖
    match matches.value_of("file") {
        Some(filename) => {
            println!("{}", filename);
            let mut file = File::create(filename).expect("unable to create file");
            file.write_all(text.as_bytes())
                .expect("unable to write data to file");
        }
        None => {
            if omit_newline {
                print!("{}", text);
            } else {
                println!("{}", text);
            }
        }
    }
}
