struct User { // Struct
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32); // Tuple Struct

fn main() {
    let mut user = get_user(String::from("MyeongSeok Ko"), String::from("myeongsku@gmail.com"));
    
    user.active = false; // assign value to mutable struct's field.

    let user2 = User { // Using struct update syntax
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };

    let black = Color(0, 0, 0);
}

fn get_user(email: String, username: String) -> User {
  User { // Using field init shorthand
    email,
    username,
    sign_in_count: 10,
    active: true
  }
}
