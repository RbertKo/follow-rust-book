fn main() {
    let some_u8_value: Option<u32> = Some(3);
    
    // Using match
    match some_u8_value {
        Some(3) => println!("Oh, this is three."),
        _ => println!("Can't read this value.")
    }

    // Using if let syntax
    if let Some(3) = some_u8_value {
        println!("Oh, this is three.");
    } else {
        println!("Can't read this value.");
    }
}
