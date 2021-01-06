fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // This(&) is called 'reference'

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // This(&) is called 'borrow'
    s.len()
}