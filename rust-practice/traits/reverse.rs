#[derive(Debug)]
struct V {
    data: Vec<u32>,
}

impl V {
    fn new() -> Self {
        V { data: Vec::new() }
    }
    fn reverse(&self, data: &Vec<u32>) -> Self {
        let mut n = V::new();
        n.data = data.clone();
        n.data.reverse();
        n
    }
}

// Example usage
fn main() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5];

    let mut new_v = V::new();
    new_v = new_v.reverse(&v);
    println!("Original: {:?}", v);
    println!("Reversed: {:?}", new_v);
}
