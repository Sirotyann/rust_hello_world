
/***
*
* For types that implement the Copy trait, like i32,
*   the values are copied into the hash map.
*
* For owned values like String,
*   the values will be moved and the hash map will be the owner of those values
*
***/
use std::collections::HashMap;

pub fn demo() {
    println!("###### <<< Rust hashmap ######");
    demo_create();
    demo_loop();
    println!("###### Rust hashmap >>> ######");
    println!("");
}

fn demo_create() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    println!("{} team score: {}", team_name, scores.get(&team_name).unwrap());
}

fn demo_loop() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
