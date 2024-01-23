use std::io;

fn main() {
    println!("Hello, user!");

    println!("What number am I thinking?");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line");

    println!("Your guess: {guess}");
}
