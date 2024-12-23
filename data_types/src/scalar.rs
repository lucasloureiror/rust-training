/*
Rust has 4 scalar types: integers, floats, booleans, and char.

I. Integers: Can be signed or unsigned (unsigned cannot have negative values).
    signed: Range from -2^n-1 to 2^n-1 - 1, where n is the number of bits.
    unsigned: Range up to 2^n-1, where n is the number of bits.
    1. 8-bit: i8 for signed and u8 for unsigned.
    2. 16-bit: i16 for signed and u16 for unsigned.
    3. 32-bit (default): i32 for signed and u32 for unsigned.
    4. 64-bit: i64 for signed and u64 for unsigned.
    5. 128-bit: i128 for signed and u128 for unsigned.
    6. Arch (depends on the processor architecture): isize for signed and usize for unsigned.

II. Floating Point: Includes only 32-bit and 64-bit types, and they are always signed.
    1. 32-bit: f32 - Lower precision, takes up less space, with slight performance gains.
    2. 64-bit (default): f64 - Higher precision with a slight impact on performance.
*/
pub fn init() {
    //starts with underscore because it's not used
    let _integer: i128 = 1010; //explicit declaration

    let _answer: bool = true;

    let _character: char = 'L';

    println!("Scalars initialized!")
}
