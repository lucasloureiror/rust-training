mod collections;
mod strings;

fn main() {
    collections::vectors::main();
    strings::main("Hello,");
    collections::hash_maps::init();
}
