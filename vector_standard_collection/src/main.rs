fn main() {
    let vec: Vec<i32> = Vec::new();

    let used_vec = vec![1, 2, 3];

    let mut push_vec = Vec::new(); // It return error. if you not push some element.

    push_vec.push(1); 

    {
        let vec_scope = vec![1, 2, 3];
    } // free vec_scope and all elements
}
