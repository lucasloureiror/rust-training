use std::vec;

fn init() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    println!("Initialized Vector 1");

    let vector_two = vec![4, 5, 6]; //Rapid vector initialization
    println!("Initialized Vector 2");

    println!("Iterating through vector 1");
    iterating(&vector);
    println!("Iterating through vector 2");
    iterating(&vector_two);
}

pub fn main() {
    init();
}
//Borrowing the vec for read-only
fn iterating(vec: &Vec<i32>) {
    for i in vec {
        println!("value:{i}")
    }
}
