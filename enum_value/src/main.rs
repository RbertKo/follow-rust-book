#[derive(Debug)]
enum IpAddrKind {
    V4, // This is called Variants of enums in rust
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?} / six: {:?}", four, six);
}
