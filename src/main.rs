fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy!");
    println!("{}", is_palindrome("Rotor"));
}

fn is_palindrome(word: &str) -> bool {
    let reversed: String = word.chars().rev().collect();
    match reversed.to_lowercase() == word.to_lowercase() {
        true => true,
        false => false,
    }
}
