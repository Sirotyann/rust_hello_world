
// let
// let mut
// shadow
// const
// static

const CON: &str = "I am a const string.";
static STA: &str = "I am a static string.";

pub fn demo () {
    println!("###### <<< Demo let/let mut/shadow/const/static ######");

    let x = 5;
    println!("The value of x is: {}", x);
    // // cannot change value of x
    // x = 6;
    // println!("The value of x is: {}", x);

    // shadow
    let mut x = 6;
    println!("The value of x is: {}", x);

    x = 7;
    println!("The value of x is: {}", x);

    println!("{}", CON);
    println!("{}", STA);

    println!("###### Variables Demo >>> ######");
    println!("");
}
