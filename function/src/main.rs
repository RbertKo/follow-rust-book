fn main() {
    println!("Hello, world!");

    another_function();
    another_function_2(20);
}

// Define new functino fusing 'fn' keyword
fn another_function() {
    println!("Merry X-Mas")
}

// Add Parameters
fn another_function_2(count: i32) {
    println!("{}th Christmas", count)
}