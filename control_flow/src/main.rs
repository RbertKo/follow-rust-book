fn main() {
    // if expressions

    let number = 5;

    let number = if number < 3 { // like ternary operator
        println!("number is less than 3: number = {}", number);
        number
        // "number" => Error! because, it isn't not same return type with 'else'
    } else {
        println!("number is greater than 3: number = {}", number);
        3
    };

    // if number { => Error. Rust will not automatically try to convert non-Boolean types to a Boolean.
    //     println!("number is existed.")
    // }
    // => Good case
    if number == 3 {
        println!("number is 3: number = {}", number);
    } else if number == 5 {
        println!("number is 5: number = {}", number);
    } else {
        println!("number is not 3 and 5. number = {}", number);
    }

    // Repetition
    // loop keyword

    let mut new_number = loop {
        println!("infinity");

        if number >= 3 { 
            break number + 1
        }
    };

    println!("new_number is 'number' + 1. new_number = {}", new_number);

    // while keyword
    while new_number < 6 {
        println!("new_number is less than 6. new_number = {}", new_number);

        new_number += 1;
    }

    // for
    let array = [10, 20, 30, 40, 50];
    
    for i in (0 .. array.len()).rev() {
        println!("i is {}", array[i])
    }
}
