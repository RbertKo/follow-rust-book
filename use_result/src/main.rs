use std::fs::File;
use std::io::ErrorKind;
use std::env;

fn main() {
    let current_dir = env::current_dir();
    
    let current_dir = if let Ok(path) = &current_dir { 
        let result = if let Some(str_path) = path.to_str() {
            str_path
        } else {
            panic!("It's not current working dir");
        };

        format!("{}/{}", result, "hello.txt")
    } else {
        panic!("panic when get path {:?}", &current_dir);
    };

    println!("{:?}", current_dir);

    let f = File::open(&current_dir).unwrap();

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create(&current_dir) {
    //             Ok(fc) => fc,
    //             Err(created_error) => {
    //                 panic!("파일을 생성하는 도중 문제가 발생했습니다: {:?}", created_error);
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         panic!("파일을 여는 도중 문제가 발생했습니다: {:?}", error)
    //     }
    // };

    println!("file is '{:?}'", f);
}
