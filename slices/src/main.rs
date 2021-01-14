fn main() {
    let s = String::from("Hello, world!");
    let s2 = &s[..];

    let len = first_word(&s);

    let hello = &s[..len];
    let world = &s[7..];

    let s = "&s.push('s');";

    println!("{}", hello);
    println!("{}", world);
    println!("{}", &s[..]); // a slice of the entire string
    println!("{}", s2)
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