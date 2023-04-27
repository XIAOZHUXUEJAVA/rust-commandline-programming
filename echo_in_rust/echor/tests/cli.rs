use std::fs; // 提供文件系统相关的操作

use assert_cmd::Command; // 命令行输出的测试

use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]

// 没有参数的情况下会运行失败
// 使用了 Command 类型来运行名为 echor 的命令行程序，并检查该程序是否在没有参数的情况下失败，
// 同时 stderr 是否包含 "USAGE" 字符串。如果测试通过，则返回一个空元组 () 包装在 Result 类型中。
// 如果测试失败，则会抛出一个错误，该错误被包装在 Box<dyn std::error::Error> 类型中，并返回一个 Result 类型对象。
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")? // cargo_bin 从rust二进制文件夹中找 ? -> 有可能找不到
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}
// 在 Rust 中，? 符号用于处理错误。
// 它可以将返回 Result 类型的函数的结果进行简化，如果执行成功，它会自动提取 Ok 中的值并返回，
// 否则它将通过 return 关键字提前从当前函数中返回一个 Err 对象。

fn run(args: &[&str], expected_file: &str) -> TestResult { // &str 类型的数组
    let expected = fs::read_to_string(expected_file)?; // 注意问号
    Command::cargo_bin("echor")? // 注意问号
        .args(args)
        .assert()
        .success()
        .stdout(expected); // 和我们的标准输出expected是否一样
    Ok(())
}
#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}
#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}
#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}


