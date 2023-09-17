#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

pub fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // iter - iterate over references
    // into_iter - turn the thing into an iterator, thus iterate over owned values
    // iter_mut - iterate over mutable references

    assert_eq!(v1.iter().sum::<i32>(), 6);
    assert_eq!(v1.iter().map(|x| x + 1).collect::<Vec<i32>>(), vec![2, 3, 4]);
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}
