fn main() {
    let s1: String = String::new();

    let s2: &str = "hello world";

    let s3 = s2.to_string(); // Declare s3: String from s2

    let s4 = String::from("like a declaring s3 from s2");

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    let mut s5 = String::from("foo");
    s5.push_str("bar");
}
