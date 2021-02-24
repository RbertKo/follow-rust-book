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
    s5.push('1');

    let s6 = String::from("hello, ");
    let s7 = String::from("world!");

    let s8 = s6 + &s7;

    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");
    let s12 = s9 + "-" + &s10 + "-" + &s11; // It's sucks

    let s13 = String::from("tic");
    let s14 = format!("{}-{}-{}", s13, s10, s11);
}
