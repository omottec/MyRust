// Storing Keys with Associated Values in Hash Maps

fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score:{:?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores1 : HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    // println!("field_name:{}, field_value:{}", field_name, field_value)

    let mut scores2 = HashMap::new();
    // Overwriting a Value
    scores2.insert(String::from("Blue"), 5);
    println!("{:?}", scores2);
    scores2.insert(String::from("Blue"), 10);
    println!("{:?}", scores2);

    // Only Inserting a Value if the Key Has No Value
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores2);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
