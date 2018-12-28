/***
if / loop / while / match

***/

pub fn demo() {
    println!("###### <<< Rust control flow ######");
    demo_if();
    demo_loop();
    demo_while();
    demo_for();
    demo_match(1);
    demo_match(0);
    println!("###### Rust control flow >>> ######");
    println!("");
}

fn demo_if() {
    let number = 3;

    if number != 0 {
        println!("If condition expect a bool.");
    }

    let if_numer_is_3 = if number != 0 {
        "Number is not 0"
    } else {
        "Number is 0"
    };

    println!("[Demo let and if] Is number = 3? : {}", if_numer_is_3);
}

fn demo_loop() {
    let mut count = 0;
    let last = loop {
        if count >= 10 {
            break count;
        }
        count += 1;
        println!("Loop {}", count);
    };
    println!("The return of loop is {}", last);
}

fn demo_while() {
    let mut count = 0;
    while count <= 10 {
        println!("While loop {}", count);
        count += 1;
    }
}

fn demo_for() {
    for i in (0..5).rev() {
        println!("For loop {}!", i);
    }
}

fn demo_match (some_u8_value:u8) {
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
