fn main() {
    let guess: u32 = "42".parse().expect("숫자가 아닙니다!");
    
    println!("guess: {}", guess);
}
