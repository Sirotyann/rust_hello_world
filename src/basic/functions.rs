/***
*
* 1) Generic data types
*
* 2) Closure
*
* 3) Closure as a param
*
* 4) Closure can capture environment:
*       -- FnOnce   :: Take ownership :: to force taking ownership, add 'move' keyword
*       -- FnMut    :: Mutably borrow
*       -- Fn       :: Borrows values from the environment immutably.
*
***/

pub fn demo () {
    println!("###### <<< Rust functions ######");
    // demo_generic
    demo_closure();
    demo_closure_as_param();
    demo_capture_env();
    println!("###### Rust functions >>> ######");
    println!("");
}
//
// fn demo_generic<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }


// -- 2) ------
fn demo_closure() {
    let plus_one = |num:u32| -> u32 {
        num + 1
    };
    // 2>  let plus_one = |num| { num + 1 }
    // 3>  let plus_one = |num|   num + 1

    let five:u32 = 5;
    println!("Five plus 1 is {}", plus_one(five));
}

// -- 3) -----
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn demo_closure_as_param() {
    let mut c = Cacher {
        calculation: |x:u32| -> u32{ x + 1 },
        value: None,
    };
    c.value(5);
    println!("cacher value {}", c.value.unwrap());
}

// -- 4) ----
fn demo_capture_env() {
    let x = 5;
    let is_equal_to_x = |it| it == x;
    let y = 4;
    println!("is {} equal to {} -- {}", y, x, is_equal_to_x(y));

    let v1 = vec![1, 2, 3];
    let is_equal_to_v1 = move |it| it == v1;
    let v2 = vec![1, 2, 3];
    // v1 and v2 are nolonger validate
    println!("{:?} is equal to v2 -- {}", vec![1, 2, 3], is_equal_to_v1(v2));
}
