/**
*
* 1) You can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
* You can even include another enum!
*
* 2) We’re also able to define methods on enums.
*
* 3) Option<T> -- for nulls
*
* 4) Match
*
**/

pub fn demo() {
    println!("###### <<< Rust enum ######");
    demo_create();
    demo_fun();
    demo_option();
    demo_match();
    demo_if_let(Some(5));
    println!("###### Rust enum >>> ######");
    println!("");
}

// --- 1) -----------------------
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn demo_create() {
    let local = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Local : {:?}", local);
    println!("Loopback : {:?}", loopback);
}


// --- 2) -----------------------
#[derive(Debug)]
enum Gender {
    MALE, FEMALE
}

impl Gender {
    fn prt(&self) {
        println!("Gender is {:?}", self);
    }
}

fn demo_fun() {
    let male = Gender::MALE;
    let female = Gender::FEMALE;
    male.prt();
    female.prt();
}


// --- 3) -----------------------
struct Person {
    first_name: String,
    last_name: Option<String>
}

fn demo_option() {
    let lc = Person {
        first_name: String::from("Cheng"),
        last_name: Some(String::from("Li"))
    };

    let lc2 = Person {
        first_name: String::from("Cheng"),
        last_name: None
    };

    println!("Name is : {} {}", lc.first_name, lc.last_name.unwrap());

    // None cannot be unwrap
    println!("Name is : {} {:?}", lc2.first_name, lc2.last_name);
    println!("Name is : {} {}", lc2.first_name, lc2.last_name.unwrap_or("No last name!".to_string()));

    // Many other methods for Option
}

// --- 4) -----------------------
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn demo_match() {
    println!("Get a Penny {}", value_in_cents(Coin::Penny));
    println!("Get a Nickel {}", value_in_cents(Coin::Nickel));
    println!("Get a Dime {}", value_in_cents(Coin::Dime));
    println!("Get a Quarter {}", value_in_cents(Coin::Quarter));
}

fn demo_if_let(num:Option<u8>) {
    if let Some(5) = num {
        println!("It is five");
    }

    if num == Some(5) {
        println!("It is five");
    }


}
