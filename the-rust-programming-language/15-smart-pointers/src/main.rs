use std::ops::Deref;
use std::rc::Rc;
use crate::{ List::*, List_::* };

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[derive(Debug)]
enum List_<T> {
    Cons_(T, Rc<List_<T>>),
    Nil_,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer{ data: String }

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    hello(&MyBox::new(String::from("Tom")));

    {
        let this = CustomSmartPointer{ data: "hello".to_string() };
        let that = CustomSmartPointer{ data: "goodbye".to_string() };
        println!("Created!");
    }

    let a = Rc::new(Cons_(5, Rc::new(Cons_(10, Rc::new(Nil_)))));
    println!("{} references", Rc::strong_count(&a));
    let b = Cons_(3, Rc::clone(&a));
    println!("{} references", Rc::strong_count(&a));
    let c = Cons_(4, Rc::clone(&a));
    println!("{} references", Rc::strong_count(&a));
}
