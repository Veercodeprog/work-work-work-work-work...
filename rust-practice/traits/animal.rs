trait Animal {
    fn make_sound(&self) -> String;
}

struct Animals {
    name: String,
}

impl Animal for Animals {
    fn make_sound(&self) -> String {
        "Moo".to_string()
    }
}

fn main() {
    let cow = Animals {
        name: "Bessie".to_string(),
    };
    println!("A cow says {}", cow.make_sound());
}
