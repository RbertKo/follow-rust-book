const YEJIN_ANNUAL_INCOME: u32 = 1200000000;

fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);

    x = 7;
    println!("x의 값: {}", x);

    println!("미래 예진이 연봉: {}", YEJIN_ANNUAL_INCOME);

    // Shadowed
    let y = 10;
    println!("y의 값: {}", y);

    // Shadowd 하다.
    let y = 12;
    println!("새로운 y의 값: {}", y);
}
