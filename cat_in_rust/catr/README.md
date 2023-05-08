## normal
```shell
$ cargo run -- tests/non_blank.txt
```

## number
```shell
$ cargo run -- -n tests/non_blank.txt
```
## number_skip_blank
```shell
$ cargo run -- -b tests/blank.txt
```

## io::stdin
### |
```shell
$ cat tests/non_blank.txt | cargo run
```

### >
```shell
$ cargo run -- < test/non_blank.txt 
```