use clap::Parser;
use std::net::{ToSocketAddrs, UdpSocket};

mod types;
use types::RecordType;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    domain: String,

    #[arg(value_enum, default_value_t = RecordType::A, ignore_case = true)]
    record_type: RecordType,

    #[arg(default_value = "1.1.1.1")]
    resolver: String,
}

fn main() {
    let args = Args::parse();

    let target = (args.resolver.as_str(), 53)
        .to_socket_addrs()
        .expect("Ungültige Adresse")
        .next()
        .expect("Keine Adresse gefunden");

    let bind_addr = if target.is_ipv6() {
        "[::]:0"
    } else {
        "0.0.0.0:0"
    };

    let socket: UdpSocket = UdpSocket::bind(bind_addr).expect("Bind fehlgeschlagen");

    let mut packet: Vec<u8> = Vec::new();

    let id: u16 = 0x1234;
    let flags: u16 = 0x0100;
    let qdcount: u16 = 1;
    let ancount: u16 = 0;
    let nscount: u16 = 0;
    let arcount: u16 = 0;

    packet.extend_from_slice(&id.to_be_bytes());
    packet.extend_from_slice(&flags.to_be_bytes());
    packet.extend_from_slice(&qdcount.to_be_bytes());
    packet.extend_from_slice(&ancount.to_be_bytes());
    packet.extend_from_slice(&nscount.to_be_bytes());
    packet.extend_from_slice(&arcount.to_be_bytes());

    for label in args.domain.split(".") {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);

    let qtype = args.record_type as u16;
    packet.extend_from_slice(&qtype.to_be_bytes());
    packet.extend_from_slice(&1u16.to_be_bytes());

    socket
        .send_to(&packet, (args.resolver.as_str(), 53))
        .expect("Senden fehlgeschlagen");

    let mut buf = [0u8; 512];

    let (anzahl, _) = socket
        .recv_from(&mut buf)
        .expect("Empfangen fehlgeschlagen");
    println!("{:02x?}", &buf[..anzahl]);
}
