// Serialization:
// Create a Serializable trait with a method to_json that returns a JSON representation of an object as a String. Implement this trait for a struct representing User and Product.
//
//

use serde_json;

trait Serializable {
    fn to_json(&self) -> String;
}
struct User {
    name: String,
    age: u32,
}

struct Product {
    id: u32,
    description: String,
}
impl User {
    fn new(name: &str, age: u32) -> User {
        User {
            name: name.to_string(),
            age,
        }
    }
}
impl Serializable for User {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };
    let user1 = User::new("Bob", 25);

    println!("User: {}", user.to_json());
}
