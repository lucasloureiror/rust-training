//Rust provides two main string types
//String: A growable, heap-allocated string and in memory it's a wrapper over Vec<u8>
//&str: An immutable view into a string, often called string slice.
//Accessing string index is not reliable, since not all UTF-8 chars occupy 1 byte.
pub fn main(input: &str) {
    //There is no mut str, so we need to create a string, since str is just a immutable view.
    let mut str = String::from(input);
    update(&mut str);
    iterate_as_bytes(&str);
    iterate_as_char(&str);
}

fn update(input: &mut String) {
    input.push_str(" World?");
}

fn iterate_as_bytes(input: &str) {
    println!("Iterating as bytes: ");
    for byte in input.bytes() {
        print!("[{byte}] ");
    }
    print!("\n");
}

fn iterate_as_char(input: &str) {
    println!("Iterating as char: ");
    for char in input.chars() {
        print!("[{char}] ");
    }
    print!("\n");
}
