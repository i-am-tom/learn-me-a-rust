fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
    fn y(&self) -> &T { &self.y }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number in {number_list:?} is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest character in {char_list:?} is {}", largest(&char_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer.x = {}", integer.x());
    println!("float.y = {}", float.y());
}
