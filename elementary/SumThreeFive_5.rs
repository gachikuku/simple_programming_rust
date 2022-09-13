use std::{io, io::Write};

fn main() {

    print!("Give me number n: ");

    io::stdout()
        .flush()
        .unwrap();

    let mut user_n = String::new();

    io::stdin()
        .read_line(&mut user_n)
        .unwrap();

    let n: i32 = user_n
        .trim()
        .parse()
        .unwrap();

    let result = (1..=n)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |a, b| a + b);

    println!("Sum: {result}");
}
