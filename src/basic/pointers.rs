/***
* 1) Lifetime. Every reference in Rust has a lifetime
*       -- Lifetime annotations don’t change how long any of the references live.
*       -- Functions can accept references with any lifetime by specifying a generic lifetime parameter.
*       -- Lifetime annotations describe the relationships of the lifetimes of multiple references
*       --  to each other without affecting the lifetimes.
*
***/

pub fn demo() {
    println!("###### <<< Rust pointers ######");

    demo_life_time();

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
