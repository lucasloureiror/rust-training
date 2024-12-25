fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("Result for largest number: {result}");
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("Result for largest char: {result}");

}
//Traits are like interfaces in rust, here I limit my generic T to a type that implement the PartialOrd trait, that allows ordering.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
