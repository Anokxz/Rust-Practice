#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

fn main() {
    let ips = vec![
        IPAddr::V4(String::from("127.0.0.1")), 
        IPAddr::V6(String::from("::-1")),
    ];

    for ip in &ips {
        match ip {
            IPAddr::V4(address) => println!("The IPv4 address is {}", address),
            IPAddr::V6(address) => println!("The IPv6 address is {}", address),
        }
    }
}
