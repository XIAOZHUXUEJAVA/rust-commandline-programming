# Rust-CommandLine-Programming
使用Rust简单实现常见的命令行命令

## echor
**version 1 输出字符串到终端界面**
类似于
新行
```shell
$ echo hello world
```
不打印新行
```shell
$ echo -n hello world
```
该程序
```shell
$ cargo run -- hello world 
```
```shell
$ cargo run -- -n hello world 
```
## echo
**version 2 添加写入内容到文件，存在一定问题, 作废**
## echo_v2
**version 3 写入功能完善**
类似于
```shell
$ echo hello world > output.txt
```
```shell
$ echo hello world >> output.txt
```
覆盖写入 
```shell
$ cargo run -- -o -f output.txt hello world
```
追加写入
```shell
$ cargo run -- -f output.txt hello world
```
## echo_v3
version 3 输出变量值, 待完善

类似
```shell
$ echo $PATH 
```
