fn main() {
    let x = 5u8;
    let y = "Hello".to_string();
    // static_dispatch(x);
    // dynamic_dispatch(&y);
}
// Exercise 6
// Fix errors and implement
// Run tests
// Hint: Associated Type

trait Container {
    type Item;
    fn insert(&mut self, item: Self::Item);
    fn remove(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
}

struct Stack {
    items: Vec<u8>,
}

impl Container for Stack {
    type Item = u8;
    fn insert(&mut self, item: Self::Item) {
        self.items.push(item)
    }
    fn remove(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
