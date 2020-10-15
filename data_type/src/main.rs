fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // 8bit Integer
    let min_int_8: i8 = i8::MIN;
    let max_int_8: i8 = i8::MAX;
    
    println!("8bit Integer is a value between {} and {}", min_int_8, max_int_8);

    // 16bit Integer
    let min_int_16: i16 = i16::MIN;
    let max_int_16: i16 = i16::MAX;

    println!("16bit Integer is a value between {} and {}", min_int_16, max_int_16);

    // 32bit Integer
    let min_int_32: i32 = i32::MIN;
    let max_int_32: i32 = i32::MAX;

    println!("32bit Integer is a value between {} and {}", min_int_32, max_int_32);

    // isize Integer => It changes depending on the memory you use.
    let min_int_isize: isize = isize::MIN;
    let max_int_isize: isize = isize::MAX;

    println!("isize Intefer is a value between {} and {}. It Changes depending on the memory you use.", min_int_isize, max_int_isize); // My System used 64bit

    // floating point. Rust's default is f64.
    let any_float = 1.0;

    print_type_of(&any_float); // f64

    let f32_float: f32 = 1.0;

    print_type_of(&f32_float);
}
