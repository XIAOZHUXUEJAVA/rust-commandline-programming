use clap::{App, Arg};
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
fn main() -> Result<(), std::io::Error> {
    let matches = App::new("echo")
        .version("0.1.0") // cargo run -- --version or cargo run -- -V
        .author("xiaozhu xiaozhuzhulzq@163.com")
        .about("Rust echo")
        .arg(Arg::with_name("string").multiple(true).required(false))
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
        .arg(
            Arg::with_name("env")
                .short("e")
                .long("env")
                .takes_value(true)
                .value_name("ENV_VAR"),
        )
        .get_matches();
   
    let text = matches
        .values_of("string")
        .map(|values| values.collect::<Vec<_>>().join(" "))
        .unwrap_or_default(); // 没有找到任何返回值的时候，返回一个空的String
    // text 不为空  执行 write and print
    if !text.is_empty() {
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
    } else if matches.is_present("env") { // text 为空 判断是否执行 echo env 
        let env_var = matches.value_of("env").unwrap();
        match std::env::var(env_var) {
            Ok(value) => {
                println!("{}={}", env_var, value);
            }
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to get environment variable: {}", e),
                ));
            }
        }
    }
    Ok(())
}
