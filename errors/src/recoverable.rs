use std::io;
pub fn main(){
    //There is a Result enum that returns (Ok(T), Err(E))
    //We can use Match or if let to deal with this type
    if let Ok(result) = return_ok() {
        println!("Success returned okay: {result}");        
    }

    if let Err(result) = return_error(){
        println!("Returned error correctly: {result}");

    }
}

fn return_error()-> Result<String, io::Error>{
    let custom_error = io::Error::new(io::ErrorKind::Other, "Custom error occurred");
    Err(custom_error) // Return an Err variant with the custom error
}

fn return_ok()-> Result<String, io::Error>{
    let okay = String::from("Returned okay");
    Ok(okay)
}