pub fn init() {
    // By default, variables in Rust are immutable!
    // Declaring a variable is done with `let`, even though they are immutable by nature
    let x = 5;
    println!("The value of x is {x}");

    // Creating a mutable variable (mut)
    let mut y = 10;
    println!("The mutable value of y is now {y}");
    y = 1010;
    println!("The mutable value of y is now {y}");

    // Declaring a constant, which is conventionally written in uppercase!
    const FIXA: i32 = 10;
    println!("Here I have a constant with value {FIXA}");
    // Shadowing: Declaring a new variable with the same name as a previous one.
    let x = "Now I am a String!";
    println!("I changed the variable x, which was immutable, using shadowing to: {x}");
}
