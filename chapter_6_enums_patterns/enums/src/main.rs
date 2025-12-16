enum IpAddrString {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct Ipv4Addr {
    address: (u8,u8,u8,u8),
    description: String
}

struct Ipv6Addr {
    address: String,
    description: String
}

impl Default for Ipv4Addr {
    fn default() -> Ipv4Addr{
        Ipv4Addr{
            address: (0,0,0,0),
            description: String::from("IP V4 Address"),
        }
    }
}

impl Default for Ipv6Addr {
    fn default() -> Ipv6Addr{
        Ipv6Addr{
            address: String::from(""),
            description: String::from("IP V4 Address"),
        }
    }
}

fn main() {
    let _home = IpAddrString::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrString::V4(String::from("::1"));
    let _home = IpAddr::V4(Ipv4Addr{address:(0,0,0,0), description: "".to_string()});
    let _home = IpAddr::V6(Ipv6Addr{address:"foo".to_string(), description: "bar".to_string()});
}
