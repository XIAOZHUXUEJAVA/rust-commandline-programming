use clap::{App, Arg};
use std::fs::OpenOptions;
use std::io::Write;
fn main() -> Result<(), std::io::Error> {
    let matches = App::new("echo")
        .version("0.1.0") // cargo run -- --version or cargo run -- -V
        .author("xiaozhu xiaozhuzhulzq@163.com")
        .about("Rust echo")
        .arg(Arg::with_name("string").multiple(true).required(true))
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("out_put")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("overwrite")
                .short("o")
                .help("Overwrite All Text in Specific File")
                .takes_value(false),
        )
        .get_matches();
    let text = matches
        .values_of("string")
        .unwrap()
        .collect::<Vec<_>>()
        .join(" ");
    let omit_newline = matches.is_present("omit_newline");
    let overwirte = matches.is_present("overwrite");

    match matches.value_of("file") {
        Some(filename) => {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(overwirte) // overwrite 为 true，可以覆盖, 所以不可以追加 , writeln覆盖
                .append(!overwirte) // overwrite 为 false，不可以覆盖, 所以可以追加 writeln() 追加
                .open(filename)?;
            writeln!(file, "{}", text)?;
        }
        None => {
            if omit_newline {
                print!("{}", text);
            } else {
                println!("{}", text);
            }
        }
    }
    Ok(())
}
