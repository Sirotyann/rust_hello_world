/***
*
* 1) Lifetime. Every reference in Rust has a lifetime
*       -- Lifetime annotations don’t change how long any of the references live.
*       -- Functions can accept references with any lifetime by specifying a generic lifetime parameter.
*       -- Lifetime annotations describe the relationships of the lifetimes of multiple references
*       --  to each other without affecting the lifetimes.
*
* ## Smart pointers are data structures that not only act like a pointer but also have additional ##
* ## metadata and capabilities. ##
* ## e.g.: String, vec<T>, Box ##
*
* 2) Box<T>.
*       -- Boxes allow you to store data on the heap rather than the stack.
*
* 3) Deref trait. ( DerefMut for mut, or &mut T will be coercion to &U while T: Deref<Target=U> )
*       -- After implement the Deref trait, dereferencing with the * operator is enable.
*       -- sync: deref(&self) -> &T
*
* 4) Deref coercion.
*
***/

pub fn demo() {
    println!("###### <<< Rust pointers ######");

    demo_life_time();
    demo_box_base();
    demo_deref();
    demo_deref_coercion();

    println!("###### Rust pointers >>> ######");
    println!("");
}

// --- 1) ----
fn demo_life_time() {
    let str1 = "hello";
    let str2 = "good morinig";

    let longer = longest(&str1, &str2);
    println!("Longer string is {}", longer);
}

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters,
// both of which are string slices that live at least as long as lifetime 'a. T
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// --- 2) ----
fn demo_box_base() {
    let b = Box::new(5);
    println!("b = {} is in Heap", b);

    let x = 6;
    let y = &x;
    let z = Box::new(x);

    println!("x={}  y={} z={}", x, *y, z);
    println!("x==y {}", (x == *y));
    println!("x==z {}", (x == *z));
}

// --- 3) --- create a Box ---
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn demo_deref() {
    let box1 = MyBox::new(5);
    println!("It's box1 {:?}", box1);
    println!("box1 == 5? {}", (*box1 == 5));
}

// --- 4) --- Deref coercion
fn hello(word: &str) {
    println!("Hello, {}!", word);
}

fn demo_deref_coercion() {
    let name = MyBox::new(String::from("cli"));
    hello(&name); // equals ==> hello(&(*name))
    hello(&(*name));
}
