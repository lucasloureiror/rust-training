/*

                                    COMPOUNDS
    1. Concept: Groups variables of different types into the same structure.
    2. Tuples: A fixed-size type that groups multiple variables into a single structure.
    3. Arrays: A collection of elements of the same type with a fixed size.

*/

pub fn init() {
    // Tuple
    let tup: (i32, f64, u8) = (500, 10.10, 10);

    let (x, y, z) = tup; // Decomposing `tup` into 3 separate elements.

    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");

    let second_position = tup.1; // Accessing the second position of the `tup` structure

    println!("{second_position}");

    // Array
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _typed_array: [i32; 2] = [1, 2]; // Declaring an array with a specific type and length
    println!("Accessing position 0 of the array {}", array[0]);
}
