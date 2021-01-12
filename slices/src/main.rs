fn main() {
    let s = String::from("Hello, world!");

    first_word(&s);

    let hello = &s[..5];
    let world = &s[7..];

    println!("{}", hello);
    println!("{}", world);
}

fn first_word(s: &String) -> usize {
    let bytes: Vec<_> = s.chars().collect();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}", &item);
        
        if item == ' ' {
            println!("item: {} / result: {}", item, item == ' ');
            return i;
        }
    }

    s.len()
}