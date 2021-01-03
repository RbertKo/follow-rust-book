fn main() {
    println!("Hello, world!");

    another_function();
    another_function_2(20);

    println!("{} is five.", five());
}

// Define new functino fusing 'fn' keyword
fn another_function() {
    println!("Merry X-Mas")
}

// Add Parameters
fn another_function_2(count: i32) {
    println!("{}th Christmas", count)
}

// Function return last expression in Rust. and, if you return value, you've to specify return type. 
fn five() -> i32 {
    5
}