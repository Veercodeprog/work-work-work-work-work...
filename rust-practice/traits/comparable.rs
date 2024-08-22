use std::cmp::Ordering;
trait Comparable {
    fn compare(&self, object: &Self) -> Ordering;
}

impl Comparable for i32 {
    fn compare(&self, object: &Self) -> Ordering {
        self.cmp(object)
    }
}

fn main() {
    // Comparing i32 values
    let a: i32 = 5;
    let b: i32 = 10;
    match a.compare(&b) {
        Ordering::Less => println!("{} is less than {}", a, b),
        Ordering::Equal => println!("{} is equal to {}", a, b),
        Ordering::Greater => println!("{} is greater than {}", a, b),
    }
}
