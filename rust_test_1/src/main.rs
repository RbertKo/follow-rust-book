enum A {
  Zero,
  One,
  Two()
}

fn main() {
    println!("{} {} {}", A::Zero as usize, A::One as usize, A::Two as usize);
}
