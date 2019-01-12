
use self::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, n:i32) -> List {
        Cons(n, Box::new(self))
    }

    fn stringify(&self) -> String {
        match *self {
             Cons(head, ref tail) => {
                 // `format!` is similar to `print!`, but returns a heap
                 // allocated string instead of printing to the console
                 format!("{}, {}", head, tail.stringify())
             },
             Nil => {
                 format!("Nil")
             },
         }
    }
}

pub fn demo() {
    println!("###### <<< Rust cons list ######");

    let list = Cons(1,
            Box::new(Cons(2,
                Box::new(Cons(3,
                    Box::new(Nil))))));

    let list_2 = list.prepend(9);
    println!("list is {}", list_2.stringify());

    println!("###### Rust cons list >>> ######");
    println!("");
}
