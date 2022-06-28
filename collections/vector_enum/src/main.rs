fn main() {
    let ip_address = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for i in ip_address {
        show_ip_address(i);
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn show_ip_address(ip: IpAddr) {
    println!("{:?}", ip)
}
