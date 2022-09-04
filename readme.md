## grep-lite

Rust实现的简化版grep程序

使用用法

```shell
cd grep-lite
cargo run -- <pattern> <path>
```

第1个参数为要查询的字符串

第2个参数为查询文件的路径

例如：

```shell
cargo run -- "test" "readme.md"
```

