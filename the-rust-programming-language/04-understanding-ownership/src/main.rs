fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

//     let s1 = String::from("hello");
//     let s2 = s1;
//
//     println!("{}, world!", s1);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    takes_ownership(s1);
    println!("s2 = {}", s2);

    let x = 5;
    makes_copy(x);

    gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back_ownership(s2);

    println!("The length of {s3} is {}", calculate_length(&s3));

    let huh = String::from("hello world");
    let first = first_word(&huh);

    println!("The first word is {}", first);
}

fn gives_ownership () -> String {
    String::from("yours")
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
