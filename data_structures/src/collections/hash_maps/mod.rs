use std::collections::HashMap;

pub fn init() {
    let mut kv: HashMap<&str, &str> = HashMap::new();
    insert(&mut kv);
    get(&mut kv);
}

fn insert(kv: &mut HashMap<&str, &str>) {
    println!("Inserting 2 itens inside HashMap");
    kv.insert("Key 1", "Value 1");
    kv.insert("Key 2", "Value 2");
    //Try to insert at key 2 if it's empty, so it does not change anything.
    kv.entry("Key 2").or_insert("Value 3");
}

fn get(kv: &mut HashMap<&str, &str>) {
    if let Some((key, value)) = kv.get_key_value("Key 2") {
        println!("Value for {key} is {value}");
    }
    println!();
}
