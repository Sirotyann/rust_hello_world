/**
Rust calls drop automatically at the closing curly bracket.

scope / move / borrow / clone

Borrow:
1) You can have only one mutable reference to a particular piece of data in a particular scope.

**/
pub fn demo() {
    println!("###### <<< Rust ownership ######");

    demo_scope();
    demo_move();
    demo_clone();
    demo_copy();

    let s1 = String::from("A string pass to a function");
    let s2 = takes_ownership(s1);
    println!("s2 = {}", s2);

    let mut s3 = String::from("A mut string ");
    borrow_mut_ownership(&mut s3);
    println!("s3 {}", s3);

    println!("###### Rust ownership >>> ######");
    println!("");
}

fn takes_ownership(some_string: String) -> String { // some_string comes into scope
    println!("function takes ownership {}", some_string);
    some_string // return ownership
}

fn borrow_mut_ownership(some_string: &mut String) {
    some_string.push_str(" has been changed");
}

fn demo_scope() {
    {
        let s = String::from("Hello");
        println!("string can be accessed inside scope. {}", s);
    }
        // println!("string cannot be accessed outside scope. {}", s);
}

fn demo_move() {
    let s1 = String::from("demo move");
    let s2 = s1;
    println!("String has been moved from s1 to s2. {}", s2);
    println!("s1 is not available");
}

fn demo_clone() {
    let s1 = String::from("demo clone");
    let s2 = s1.clone();
    println!("Clone from s1 to s2. s1: {}, s2: {}", s1, s2);
}

fn demo_copy() {
    println!("Copy is only for stack");
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
