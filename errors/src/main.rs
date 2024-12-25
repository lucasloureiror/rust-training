mod panic;
mod recoverable;

fn main() {
    recoverable::main();
    panic::main();
}
