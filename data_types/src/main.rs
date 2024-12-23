mod compounds;
mod declare;
mod scalar;

fn main() {
    scalar::init();
    declare::init();
    compounds::init();
}
