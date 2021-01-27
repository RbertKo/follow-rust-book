#[derive(Debug)]
enum IpAddrKind {
    V4(String), // This is called Variants of enums in rust
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    
    let six = IpAddrKind::V6(String::from("::1"));

    println!("four: {:?} / six: {:?}", four, six);
}
