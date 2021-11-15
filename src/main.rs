use std::collections::HashMap;
use std::env;
use std::io;
use std::fs;

fn main() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo {
        map: HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred {}", why),
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), io::Error> {
        let mut content = String::new();
        for (k,v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        fs::write("db.txt", content)
    }
    // TODO: implement new() method
}