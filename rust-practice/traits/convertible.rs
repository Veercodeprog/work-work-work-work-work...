trait convertible<T> {
    fn convert(&self) -> T;
}

impl convertible<String> for i32 {
    fn convert(&self) -> String {
        self.to_string()
    }
}

impl convertible<i32> for String {
    fn convert(&self) -> i32 {
        self.parse().unwrap()
    }
}

fn main() {
    let a: i32 = 5;
    let b = a.convert();
    println!("{}", b);
    let c = b.convert();
    println!("{}", c);
}
