use std::{collections::HashMap};

struct KeyValueStore{
    store: HashMap<String, String>
}

struct Primary{
    pkvstore: KeyValueStore
}

struct Secondary{
    skvstore:KeyValueStore
}

impl Primary{
    fn insert(&mut self, k:&str, v:&str)-> Option<String>{
        self.pkvstore.insert(k,v)

    }

    fn replicate(sec: &mut Secondary, k: &str, v: &str){
        sec.insert(k,v);
    }
}

impl Secondary{
    fn insert(&mut self, k: &str, v: &str)->Option<String>{
        self.skvstore.insert(k,v)
    }
}

impl KeyValueStore{
    fn new()->KeyValueStore{
        KeyValueStore { store: (HashMap::new()) }
    }
    
    fn insert(&mut self, k: &str, v: &str)->Option<String>{
        self.store.insert(k.to_string(),v.to_string()) /*Here k and v are kept as string slices so that the primary processes still owns them after the insert. This is to facilitate communication b/w primary and secondary. Then while 
        inserting we can use the .to_string() function to convert a string slice into an owned string.*/
    }

    fn get(&self, k: &str)->Option<&String>{
        self.store.get(k) //here k is &str because we don't need to own it. We are just using it to query
        //Also we are returning Option<&String> so as to not transfer ownership of the string that is present 
        //in the value
    }
}

fn main() {
    
    let mut foo = KeyValueStore::new();
    let _presence_of_key=foo.insert("Sparsh", "Kota");

    foo.insert("Sparsh","Jaipur");
    
    match foo.get("Sparsh"){
        Some(x)=>println!("The value is {x}"),
        None=>println!("No value found")
    }
    
}
