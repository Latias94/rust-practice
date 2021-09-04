# 环境搭建

## leetcode

[leetcode 环境设置](./leetcode/README.md)

## project readme

安装 diesel cli:
```shell
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

安装 [sagiegurari/cargo-make](https://github.com/sagiegurari/cargo-make):

```shell
cargo install --force cargo-make
```

抓取题目、描述和解答到数据库，并根据模板生成 README.md:

```shell
cargo make readme
```
