use std::fs::File;

fn main() {
    let f = File::open("./hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("파일을 여는 도중 문제가 발생했습니다: {:?}", error)
        }
    };

    println!("file is '{:?}'", f);
}
