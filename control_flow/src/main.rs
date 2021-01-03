fn main() {
    // if expressions

    let number = 5;

    if number < 3 {
        println!("number is less than 3: number = {}", number)
    } else {
        println!("number is greater than 3: number = {}", number)
    }

    // if number { => Error. Rust will not automatically try to convert non-Boolean types to a Boolean.
    //     println!("number is existed.")
    // }
    // => Good case
    if number != 3 {
        println!("number is not 3: number = {}", number);
    }
}
