# init-nodejs-project（inp）

快速创建 node.js 项目，并内置 eslint、prettier、editorconfig 配置。

## 使用

```sh
# 显示命令行参数
$ init-nodejs-project.exe --help
    -a, --author
            设置模块作者

    -h, --help
            显示帮助信息

    -n, --name
            设置模块名称，并创建同名目录

    -p, --private
            设置为私有模块

    -v, --version
            显示版本号

# 创建项目目录（通过 init-nodejs-project -n myproject 可以自动创建目录）
$ mkdir myproject
$ cd myproject

# 用默认设置初始化项目
$ init-nodejs-project 

# 安装依赖
$ npm i

# 项目完成初始化
```

## 设置

可以在环境变量中增加 `INP_AUTHOR` 来指定模块作者。

## License

[MIT](LICENSE)
