use std::{collections::HashMap};

struct KeyValueStore{
    store: HashMap<String, String>
}

impl KeyValueStore{
    fn new()->KeyValueStore{
        KeyValueStore { store: (HashMap::new()) }
    }
    
    fn insert(&mut self, k: String, v: String)->Option<String>{
        self.store.insert(k,v)
    }
}

fn main() {
    let mut foo = KeyValueStore::new();
    let mut presence_of_key=foo.insert(String::from("Sparsh"), String::from("Kota"));
    // let presence_of_key = foo.insert(String::from("Sparsh"), String::from("Jaipur"));

    match presence_of_key{
        Some(..)=>println!("replaced"),
        None=>println!("created")
    }
    presence_of_key=foo.insert(String::from("Sparsh"), String::from("Jaipur"));

    match presence_of_key{
        Some(..)=>println!("replaced"),
        None=>println!("created")
    }
    
}
