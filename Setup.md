# 环境搭建

## leetcode

[leetcode 环境设置](./leetcode/README.md)

## project readme

安装 diesel cli:
```shell
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

安装 [sagiegurari/cargo-make](https://github.com/sagiegurari/cargo-make)

```shell
cargo install --force cargo-make
```

生成数据库表

```shell
cargo make --makefile .\Makefile.toml readme
```

再运行一次 `rustgym-readme` 生成 README，并存数据到数据库中。