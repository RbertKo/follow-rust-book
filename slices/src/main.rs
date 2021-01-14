fn main() {
    let s = String::from("Hello, world!");

    let s2 = &s[..];
    

    let len = first_word(&s);

    let hello = &s[..len];
    let world = &s[7..];

    let s = "sdf".to_string();

    println!("{}", hello);
    println!("{}", world);
    println!("{}", &s[..]); // a slice of the entire string
    println!("{}", s2);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{:?}", slice);
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