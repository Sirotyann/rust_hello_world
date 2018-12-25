
// let
// let mut
// const
// shadow

pub fn demo () {
    println!("###### <<< Demo let/let mut/const/shadow/ ######");

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

    println!("###### Variables Demo >>> ######");
    println!("");
}
