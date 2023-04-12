# Rust 全栈 Web 开发

## 技术栈

- Web Service
- 服务端 Web App
- 客户断 Web App （WebAssembly）
- Web 框架：Actix Web
- 数据库：PostgreSQL
- 数据库连接：SQLx

## 项目结构

- 使用 Cargo Workspace 管理多个项目

## Run

```bash
# 启动 tcpserver
cargo run -p tcpserver

# 启动 tcpclient
cargo run -p tcpclient
```

## HTTP Server Map

- Server TCP -> HTTP Library -> HTTP Server -> HTTP Router -> HTTP Handler
