use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    let k = String::from("favorite color");
    let v = String::from("blue");
    let mut map = HashMap::new();
    map.insert(k, v);
    // println!("{k}");

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 50);

    scores.entry(String::from("red")).or_insert(20);
    scores.entry(String::from("blue")).or_insert(30);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
