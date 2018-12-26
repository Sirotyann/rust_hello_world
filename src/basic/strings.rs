
pub fn demo() {
    println!("###### <<< Rust String ######");
    let s = String::from("Hello world");
    println!("String after slice [0..=3] is {}", &s[0..=3]);
    println!("###### Rust String >>> ######");
}
