# 环境搭建

## leetcode

[leetcode 环境设置](./leetcode/README.md)

## project readme

根据题目生成 README.md，模板在 `readme/templates/readme.j2`。

### 安装工具

安装 diesel cli:
```shell
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

安装 [sagiegurari/cargo-make](https://github.com/sagiegurari/cargo-make):

```shell
cargo install --force cargo-make
```

抓取题目、描述和解答到数据库，并根据模板生成 README.md:

### 生成

1. 每次构建生成：
    ```shell
    cargo make readme
    ```

2. 每次通过构建好的二进制生成：
    ```shell
    cargo build --release
    ```
    复制 target\release\rustgym-readme.exe 到项目根目录。
    
    以后就能直接执行二进制生成 README.md 避免重复构建。
    
    ```shell
    cargo make readme_no_build
    ```