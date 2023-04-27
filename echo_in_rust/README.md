## Echo-In-Rust
使用Rust简单实现echo命令行命令

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

## echor
**version 1 输出字符串到终端界面**
类似于以下`echo`命令
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
**用于开发**

## echo_v2
**version 2 在打印的功能上添加写入功能**

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
version 3 输出变量值

类似
```shell
$ echo $PATH 
```

```shell
$ cargo run -- -e PATH
$ cargo run -- -e PNPM_HOME
```

