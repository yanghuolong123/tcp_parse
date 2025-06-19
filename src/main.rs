use pnet::{
    datalink::{self},
    packet::{
        Packet,
        ethernet::{EtherTypes, EthernetPacket},
        ip::IpNextHeaderProtocols,
        ipv4::{ Ipv4Packet},
        tcp::TcpPacket,
    },
};

fn main() {
    // 获取所有网络接口
    let interfaces = datalink::interfaces();

    // 选择第一个非回环、正在运行的接口
    let interface = interfaces
        .into_iter()
        .find(|iface| !iface.is_loopback() && iface.is_up())
        .expect("找不到可用的网络接口");

    println!("使用网络接口:{}", interface.name);

    // 创建抓包通道
    let mut rx = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(_, rx)) => rx,
        Ok(_) => panic!("不支持的通道类型"),
        Err(e) => panic!("无法创建通道:{}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let eth_packet = match EthernetPacket::new(packet) {
                    Some(p) => p,
                    None => continue,
                };

                if eth_packet.get_ethertype() == EtherTypes::Ipv4 {
                    if let Some(ipv4) = Ipv4Packet::new(eth_packet.payload()) {
                        if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                            if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                println!(
                                    "TCP包: {}:{} -> {}:{} | seq={} Ack={} Len={}",
                                    ipv4.get_source(),
                                    tcp.get_source(),
                                    ipv4.get_destination(),
                                    tcp.get_destination(),
                                    tcp.get_sequence(),
                                    tcp.get_acknowledgement(),
                                    tcp.payload().len()
                                );

                                print_payload(tcp.payload());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("读取数据包错误：{}", e)
            }
        }
    }
}

// 将payload打印为十六进制和ASCII(仅打印前512字节，防止过长)
fn print_payload(payload: &[u8]) {
    const MAX_LEN: usize = 512;
    let len = payload.len().min(MAX_LEN);

    if len == 0 {
        println!("  数据：<空>");
        return;
    }

    // 十六进制输出
    print!("    十六进制:");
    for byte in &payload[..len] {
        print!("{:02X}",byte);
    }
    println!();

    // ASCII输出 (不可打印字符用 . 替代)
    print!("    ASCII:");
    for byte in &payload[..len] {
        if byte.is_ascii_graphic() || *byte == b' ' {
            print!("{}", *byte as char);
        } else {
            print!(".")
        }
    }

    parse_http(payload);

    println!("\n===================================\n")
}

/// 判断并解析HTTP报文
fn parse_http(payload: &[u8]) {
    if payload.is_empty() {
        return;
    }

    // 尝试将payload 转换成UTF-8 字符串
    if let Ok(text) = std::str::from_utf8(payload) {
        // 检查是否是HTTP 请求或响应
        if text.starts_with("GET ") || text.starts_with("POST ") || text.starts_with("HTTP/") {
            println!("---------- HTTP 数据开始 ----------");
            for line in text.lines().take(20) {
                println!("    {}",line);
                if line.is_empty() {
                    break; // 空行之后是body, 不再打印
                }
            }
            println!("---------- HTTP 数据结束 ----------\n");
        }
    }
}