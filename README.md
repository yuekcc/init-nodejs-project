# init-nodejs-project

快速创建 node.js 初始项目，并设置 eslint、prettier、editorconfig、typescript。

## 使用

```sh
# 显示命令行参数
$ init-nodejs-project.exe -h
init-nodejs-project 0.5.0-7930e41

USAGE:
    init-nodejs-project.exe [OPTIONS] [NAME]

ARGS:
    <NAME>    Set project name (and create project folder). If not parent, use working dir name
              as project name

OPTIONS:
    -a <AUTHOR>        Set author name [default: no_name]
    -h, --help         Print help information
    -p, --private      Set project as PRIVATE
    -V, --version      Print version information

# 创建项目目录（通过 init-nodejs-project -n myproject 可以自动创建目录）
$ mkdir myproject
$ cd myproject

# 用默认设置初始化项目
$ init-nodejs-project 

# 安装依赖
$ npm i

# 项目完成初始化
```

## 构建

需要 rust 1.59

```sh
just release # 或 cargo build --release
```

## License

[MIT](LICENSE)
