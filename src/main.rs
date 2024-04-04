use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }
}

fn main() {
    let mut todo = Todo {
        map: HashMap::new(),
    };

    todo.insert(String::from("놀기"));

    for i in todo.map {
        println!("{:?}", i);
    }
}
