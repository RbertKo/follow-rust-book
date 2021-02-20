fn main() {
    let vec: Vec<i32> = Vec::new();

    let used_vec = vec![1, 2, 3];

    let mut push_vec = Vec::new(); // It return error. if you not push some element.

    push_vec.push(1); 

    {
        let vec_scope = vec![1, 2, 3];

        let third: &i32 = &vec_scope[2];
        let third_option: Option<&i32> = vec_scope.get(2);

        println!("{}", third);
        println!("{}", if let None = third_option {
            0
        } else {
            vec_scope[2]
        })
    } // free vec_scope and all elements
}
