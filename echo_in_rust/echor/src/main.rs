use clap::{App, Arg}; // rust clap 是 Rust 编程语言中的一个命令行参数解析库，它可以帮助开发者在 Rust 应用程序中轻松地定义和解析命令行参数
fn main() {
    // println!("Hello, echor!");
    // 使用 cargo run hello world 测试一下
    // Args { inner: ["target/debug/echor", "hello", "world"] }
    // std::env::args()包含程序运行时所接受的所有命令行参数， 使用 {:?} 格式化输出
    // println!("接受的命令行参数: {:?}", std::env::args());

    let matches = App::new("echor") // cargo run -- -h -> for help of the app
        .version("0.1.0") // cargo run -- --version or cargo run -- -V
        .author("xiaozhu xiaozhuzhulzq@163.com")
        .about("Rust echo")
        .arg(
            Arg::with_name("text") // 参数的名称, 此参数用于输入文本，暂时还没有实现
                .value_name("TEXT") // 参数值的名称  <TEXT>...    Input text
                .help("Input text")
                .required(true) // 必须使用参数
                .min_values(1), // 可以重复，但是不能小于1次
        )
        .arg(
            Arg::with_name("omit_newline") // 用于控制是否省略输出文本末尾的换行符
                .short("n") // 表示 "-n" 参数是一个短参数，可以使用单破折号 "-" 加上参数名的方式进行传入
                .help("Do not print newline") // 该参数不需要接收值，即它本身就是一个开关: 命令行中有这个参数的时候，省略，没有的时候，不省略
                .takes_value(false),
        )
        .get_matches(); // 告诉rustApp应该解析以上参数
                        // println!("{:#?}", matches);

    // Option<Vec<String>> 类型
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

 
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    #[test]
    fn runs() {
        let mut cmd = Command::cargo_bin("echor").unwrap();
        cmd.arg("hello").assert().success();
    }
}
