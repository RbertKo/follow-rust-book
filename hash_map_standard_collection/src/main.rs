fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("blue", 10);
    scores.insert("yellow", 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores2);

    // Ownership of Hashmap
    let new_team_name = String::from("Red");
    let new_team_score = 100;

    let mut scores3 = HashMap::new();

    scores3.insert(new_team_name, new_team_score);
    //new_team_name and new_team_score are invalid at this point.

    {
        let red_team_score = scores3.get("Red");

        if let Some(red_score) = red_team_score {
            println!("red team: {}", red_score);
        }
    }

    for (key, value) in &scores2 {
        println!("{}: {}", key, value)
    }

    scores3.insert(String::from("Red"), 150);

    let red_team_score = scores3.get("Red");

    if let Some(red_score) = red_team_score {
        println!("red team: {}", red_score);
    }
}
