fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    // contains_value(&numbers, &3);
    println!("Reversed strings: {:?}", contains_value(&numbers, &3));
}

fn contains_value<T: std::cmp::PartialEq>(collection: &[T], value: &T) -> bool {
    if collection.is_empty() {
        return false;
    }
    collection.contains(value)
}
