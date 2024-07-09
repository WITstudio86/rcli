# rcli

使用 rust 编写的 cli 工具 demo , 完整注释, 可以用来作为 rust 学习过程中的简单源码阅读素材

## 环境准备

```shell
cargo add anyhow
cargo add clap --features dervie
cargo add colored
cargo add csv
cargo add serde --features derive
cargo add serde_json
```

## 使用

使用 `--help` 查看帮助信息

实例:

```shell
# 转换为 json 文件并覆盖同名文件
csv format -i assets/juventus.csv -o assets/output -t json -r
# csv 文件内容输出到终端
cargo run -- csv show -i input.csv
```

### 支持的生成类型

- csv
- json
- toml
  - 默认吧第一列的值作为 `[]` 中的字段
