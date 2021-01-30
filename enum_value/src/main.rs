#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // This is called Variants of enums in rust
    V6(String),
}

#[derive(Debug)]
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

    let move_val = Message::Move {
        x: 1,
        y: 2,
    };

    m.call();

    println!("{:?}", move_val);
}

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}