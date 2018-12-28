/***
* Rust has only one string type in the core language, str.
* The String type, which is provided by Rust’s standard library rather than coded into the core language,
* is a growable, mutable, owned, UTF-8 encoded string type.
*
***/

pub fn demo() {
    println!("###### <<< Rust strings ######");
    demo_create();
    demo_plus();
    demo_char();
    println!("###### Rust strings >>> ######");
    println!("");
}

fn demo_create() {
    let mut hello = String::from("Hello");
    hello.push_str(" world");
    println!("{}", hello);
}

fn demo_plus() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    //s1 has been moved here and can no longer be used
    //first string must not be a borrow
    let s3 = s1 + &s2;

    println!("s3 is {}", s3);
    println!("s2 is {}", s2);
    // println!("s1 is {}", s1);
}

fn demo_char() {
    let s = String::from("Hello World");
    let ch = &s[0..1];
    println!("first char of str is {}", ch);
}
