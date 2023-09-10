use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    longest(x, y)
}

pub fn main() {
    let x = String::from("abcd");
    let y = "xyz";

    let result = longest(x.as_str(), y);
    println!("The longest string is {}", result);

    let novel = String::from("Here. is some very important text.");
    let part = novel.split('.').next().expect("Could not find '.'");
    let i = ImportantExcerpt { part };
}
