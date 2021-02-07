enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(Color),
}

#[derive(Debug)]
enum Color {
  Bronze,
  Silver,
  Gold
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
      Coin::Penny => {
        println!("Lucky penny!");
        1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(color) => {
        println!("{:?} color {}$", color, 25);
        25
      },
  }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(Color::Gold));
}
