struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("MyeongSeok Ko"),
        email: String::from("myeongsku@gmail.com"),
        sign_in_count: 10,
        active: true
    }
}
