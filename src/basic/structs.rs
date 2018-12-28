/***
Struct and Function definition doesn't need ';'
But Tuple Struct needs.
***/

pub fn demo() {
    println!("###### <<< Rust struct ######");
    let mut user1 = demo_create();
    println!("User1 is {:?}", user1);
    let user2 = User{ name: String::from("Wed"), ..user1 };
    println!("User2 is {:?}", user2);
    user1.age_plus();
    println!("After age_plus, user1 is {:?}", user1);

    let point = Point(8, 9);
    println!("point is {:?}", point);
    println!("###### Rust struct >>> ######");
    println!("");
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

// Tuple struct
#[derive(Debug)]
struct Point (i32, i32);

fn demo_create() -> User {
    // let user:User = User{ name: String::from("Cheng Li"), age: 35 };
    let user:User = User::build(String::from("Cheng Li"), 35);
    user
}

impl User {
    fn age_plus(&mut self) {
        self.age += 1;
    }

    fn build(name:String, age:u8) -> User{
        User{ name: name.to_string(), age: age }
    }
}
