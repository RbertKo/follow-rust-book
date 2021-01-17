struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user = User {
        username: String::from("MyeongSeok Ko"),
        email: String::from("myeongsku@gmail.com"),
        sign_in_count: 10,
        active: true
    };
    
    user.active = false;
    // user.active = false; -> 가변 변수일 경우 다른 값 할당 가능
}
