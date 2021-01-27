#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // This is called Variants of enums in rust
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
       println!("call")
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    
    let six = IpAddrKind::V6(String::from("::1"));

    println!("four: {:?} / six: {:?}", four, six);

    let m = Message::Write(String::from("hello"));

    m.call();
}
