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

fn chip_in(wallet: Option<u32>) -> Option<u32> {
  match wallet {
    None => Some(1000),
    Some(money) => Some(money + 1000),
  }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(Color::Gold));

    let my_wallet1: Option<u32> = Some(3000);
    let my_wallet2: Option<u32> = None; 

    println!("my wallet 1: {:?}", my_wallet1);
    println!("my wallet 2: {:?}", my_wallet2);

    let my_wallet1 = chip_in(my_wallet1);
    let my_wallet2 = chip_in(my_wallet2);

    println!("my wallet 1: {:?}", my_wallet1);
    println!("my wallet 2: {:?}", my_wallet2);
}
