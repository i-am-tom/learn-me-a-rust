const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    mutations();
    constants();
    shadow_scope();
    type_changing_shadowing();
    maths();
    tuples();
    months(5);
    semicolons();
    f2c(95.0, "Oh no");
    println!("{} + 1 = {}", five(), plus_one(five()));
    boolish(true);
    loop_break();
    loop_while();
    loop_for();
}

fn mutations() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x has been mutated to be {}", x);
}

fn constants() {
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours");
}

fn shadow_scope() {
    let y = 1;
    println!("y = {y}");

    let y = y + 1;
    println!("Shadowy = {y}");

    {
        let y = y * 2;
        println!("shadowyy = {y}");
    }

    println!("Shadowy = {y}");
}

fn type_changing_shadowing() {
    let name = "thomas";
    let name = name.len();

    println!("Thomas has {name} letters");
}

fn maths() {
    println!("Some sums sum to {}", 5 * 2 + 7 % 4);
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;
    println!("{a}? I can't {b} what you {c}");
    println!("{} in a million", tup.2);
}

fn months(choice: usize) {
    let words = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    println!("Month #{choice} is {}", words[choice - 1]);
}

fn semicolons() {
    let my_favourite_integers: [i32; 7] = [2, 3, 4, 5, 3, 1, 2];
    println!("Ah, {:?}", [7; 3]);

    println!("The first degree of the major scale is {}", my_favourite_integers[0]);
}

fn f2c(temperature: f32, interjection: &str) {
    println!("{interjection}, it's {}C out!", (temperature - 32.0) * 5.0 / 9.0);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn boolish(b: bool) {
    println!("{} is boolish!", if b { "Yes" } else { "No" });
}

fn loop_break() -> i32 {
    'test: loop { break 'test 3 }
}

fn loop_while() {
    while false { println!("never!") }
}

fn loop_for() {
    for element in [1, 2, 3, 4, 5] {
        println!("{}", element);
    }
}
