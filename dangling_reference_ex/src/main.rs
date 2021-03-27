// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

fn main() {
    let x = 5;

    let r;

    {
        
        r = &x;
    }

    println!("r: {}", r);


    ////////////////
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}
