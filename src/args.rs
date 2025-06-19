use clap::Parser;
use std::net::Ipv4Addr;


/// 抓取 TCP 数据包并可过滤
#[derive(Parser, Debug)]
#[command(name="tcp-parse", version="1.0",about="一个简单的TCP抓包工具")]
pub struct Args {
    /// 网络接口名称 (如 eth0)
    #[arg(long)]
    pub iface: Option<String>,

    /// 源 IP 过滤 (IPv4)
    #[arg(long)]
    pub src_ip: Option<Ipv4Addr>,

    /// 目标 IP 过滤 (IPv4)
    #[arg(long)]
    pub dst_ip: Option<Ipv4Addr>,

    /// 源端口过滤 
    #[arg(long)]
    pub src_port: Option<u16>,

    /// 目标端口过滤
    #[arg(long)]
    pub dst_port: Option<u16>,
}