# init-nodejs-project（inp）

一个简单的工具，用于快速创建 node.js 项目。内置了 eslint、prettier、editorconfig 配置。

## 使用

```sh
# 创建项目目录
mkdir myproject
cd myproject

# 显示命令行参数
init-nodejs-project -h

# 用默认设置初始化项目
init-nodejs-project 

# 安装依赖
npm i

# 项目完成初始化
```

## 设置

可以在环境变量中增加 `INP_AUTHOR` 来指定模块作者。

## License

[MIT](LICENSE)
