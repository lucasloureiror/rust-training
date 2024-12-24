use std::io;

fn main() {
    println!("Enter dividend");
    let mut dividend = String::new();

    io::stdin()
        .read_line(&mut dividend)
        .expect("Error reading dividend");
    let dividend: i32 = dividend
        .trim()
        .parse()
        .expect("Error converting dividend to i32");

    println!("Enter divisor");
    let mut divisor = String::new();

    io::stdin()
        .read_line(&mut divisor)
        .expect("Error reading divisor");
    let divisor: i32 = divisor
        .trim()
        .parse()
        .expect("Error converting dividend to i32");

    match safe_division(dividend, divisor) {
        //If you want to discard one of the results you can use if let as seem after this
        Some(result) => println!("Result is {result}"),
        None => println!("Division by 0"),
    }

    if let None = safe_division(dividend, divisor) {
        println!("I detected with if let a division by 0")
    }

    if let Some(result) = safe_division(dividend, divisor) {
        println!("I detected with if let a result =  {result}")
    }
}

fn safe_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
