enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling {:?}", &self)
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Balabama,
    Calabama,
    Lalalalabamba,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn cents(&self) -> u8 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter(state) => {
                println!("A quarter from {:?}?!", state);
                25
            },
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some (i + 1),
    }
}

fn add_fancy_hat() {
    println!("HAT ADDED");
}

fn remove_fancy_hat() {
    println!("HAT REMOVED");
}

fn main() {
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    println!("A nickel is {} cents", Coin::Nickel.cents());
    println!("A quarter is {} cents", Coin::Quarter(UsState::Calabama).cents());

    println!("Some(5) + 1 = {:?}", plus_one(Some(5)));
    println!("None + 1 = {:?}", plus_one(None));

    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    if let 3 = dice_roll { add_fancy_hat(); }
    else if let 7 = dice_roll { remove_fancy_hat(); }
    else { () }
}
