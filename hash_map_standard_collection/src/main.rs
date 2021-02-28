fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("blue", 10);
    scores.insert("yellow", 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let socres2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", socres2);
}
