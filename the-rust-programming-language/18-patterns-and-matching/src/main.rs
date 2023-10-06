enum These<A, B> {
    This(A),
    That(B),
    These(A, B),
}

fn main() {
    let x = These::This(2);

    match x {
        // Fine if all names are bound in all patterns
        These::This(x) | These::That(x) => println!("{x}"),
        These::These(x, y) => println!("{x}, {y}")
    }

    let y = 5;

    match y {
        1 ..= 4 => println!("Under five"),
        5 .. => println!("Over five"),
        _ => println!("Uh oh")
    }

    let z = (1, 2, 3, 4, 5);

    match z {
        (x @ name, .., 5) => println!("yay {name}"),
        _ => println!("boo"),
    }

    let w = Some(4);

    match w {
        Some(x) if x % 2 == 0 => println!("EVEN"),
        Some(x) => println!("ODD"),
        None => println!("WHERE?"),
    }
}
