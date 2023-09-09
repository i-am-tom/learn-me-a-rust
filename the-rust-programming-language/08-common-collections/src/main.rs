enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // --- //

    let mut s = String::new();
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);
    s1.push('!');
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    // --- //

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    println!("{}", median(&mut vec![1, 3, 2, 12, 8, 3]));
    println!("{}", mode(&mut vec![1, 3, 2, 12, 9, 2]));
}

fn median(items: &mut Vec<i32>) -> &i32 {
    items.sort();

    match items.get(items.len() / 2 - 1) {
        Some(value) => value,
        None => &items[0]
    }
}

fn mode(items: &Vec<i32>) -> i32 {
    let mut store = HashMap::new();

    for item in items {
        *store.entry(item).or_insert(0) += 1;
    }

    let mut result: Option<(i32, i32)> = None;

    for (key, candidate) in store {
        match result {
            Some((_, biggest)) =>
                if candidate > biggest {
                    result = Some((*key, candidate))
                },
            None =>
                result = Some((*key, candidate))
        }
    }

    let unwrapped = result.unwrap_or((0, 0));
    unwrapped.0
}
