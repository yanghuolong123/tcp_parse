# TCP Parse

一个使用 Rust 编写的简单 TCP 抓包工具，支持过滤网卡、IP 和端口，并解析 HTTP 报文内容。

---

## 功能特点

- 基于 `pnet` 抓取 TCP 报文
- 支持通过命令行参数过滤：
  - 网络接口（网卡）
  - 源 IP / 目标 IP
  - 源端口 / 目标端口
- 自动打印 HTTP 请求和响应头
- 十六进制和 ASCII 格式输出 TCP payload

---

##  依赖

- Rust 1.70+
- [`clap`](https://crates.io/crates/clap)（用于命令行解析）
- [`pnet`](https://crates.io/crates/pnet)（用于数据包抓取）

---

## 项目结构
tcp-sniffer/
├── Cargo.toml
└── src/
├── main.rs # 主逻辑
└── args.rs # 命令行参数解析模块


## 使用方式

### 编译

```bash
cargo build --release


### 示例运行命令

# 抓取 eth0 网卡上所有 TCP 报文
cargo run -- --iface eth0

# 抓取来自特定 IP 的报文
cargo run -- --iface eth0 --src-ip 192.168.1.100

# 抓取发往端口 80 的 TCP 报文
cargo run -- --iface eth0 --dst-port 80


### 示例输出

监听接口: eth0
TCP包: 192.168.1.100:34567 -> 93.184.216.34:80 | seq=123456 ack=789 len=121
    十六进制:474554202f20485454502f312e310d0a486f73743a206578616d706c652e636f6d0d0a...
    ASCII:GET / HTTP/1.1\r\nHost: example.com\r\n...

---------- HTTP 数据开始 ----------
    GET / HTTP/1.1
    Host: example.com
    User-Agent: curl/7.85.0
    Accept: */*

---------- HTTP 数据结束 ----------

===================================