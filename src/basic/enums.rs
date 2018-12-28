/**
* 1) You can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
* You can even include another enum!
*
* 2) We’re also able to define methods on enums.
*
* 3) Option<T> -- for nulls :
*
**/

pub fn demo() {
    println!("###### <<< Rust enum ######");
    demo_create();
    demo_fun();
    demo_option();
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
}
