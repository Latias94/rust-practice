# 环境搭建

## leetcode

[leetcode 环境设置](./leetcode/README.md)

## project readme

运行一次 `rustgym-readme` 会在根目录生成 rust-practice.sqlite 数据库。

安装 diesel cli:
```shell
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

生成数据库表

```shell
diesel migration run
```

再运行一次 `rustgym-readme` 生成 README，并存数据到数据库中。