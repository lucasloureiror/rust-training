use std::io;
fn main() {
    println!("Hello! What's your name?");
    //a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    let mut name = String::new();
    //new is an associated function of the String type

    //The stdin function returns an instance of std::io::Stdin,
    //which is a type that represents a handle to the standard input for your terminal.
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    //Removing the extra line with trim because I used read_line.
    println!("Hello, {}!", name.trim());

    println!("Whats your age?");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    //Expect is a way to deal with errors inside read_line

    let age: u8 = age.trim().parse().expect("Please enter a number!");

    println!("Hello, {}! You are {} years old!", name.trim(), age);
}
