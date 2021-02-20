fn main() {
    let vec: Vec<i32> = Vec::new();

    let used_vec = vec![1, 2, 3];

    let mut push_vec = Vec::new(); // It return error. if you not push some element.

    push_vec.push(1); 

    {
        let vec_scope = vec![1, 2, 3];

        let third: &i32 = &vec_scope[2];
        let third_option: Option<&i32> = vec_scope.get(5);

        println!("{}", third);
        println!("{}", if let Some(element) = third_option {
            element
        } else {
            &0
        })
    } // free vec_scope and all elements

    let v1 = vec![1, 10 ,50];

    for i in &v1 {
        println!("{}", i);
    }

    let mut v2 = vec![1, 2, 3];

    for i in &mut v2 {
        *i *= 50;
        println!("{}", i);
    }
}
