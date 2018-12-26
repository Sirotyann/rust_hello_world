/**

Primitive types::
    i8, i16, i32, i64, i128, isize
    u8, u16, u32, u64, u128, usize
    f32, f64
    char
    bool

Compond types::
    arrays   [1,2,3]
    tuples  (1, true)

 */

 pub fn demo() {
     println!("###### <<< Rust types ######");
     demo_tuple();
     demo_array();
     println!("###### Rust types >>> ######");
     println!("");
 }

 fn demo_tuple() {
     println!("-- Tuple: --");
     let tup: (i32, f64, u8) = (500, 6.4, 1);
     let (_x, _y, _z) = tup;
     println!("The value of y is: {}, the first value is {}", _y, tup.0);
 }

 fn demo_array() {
     let arr: [i32; 5] = [1, 2, 3, 4, 5];
     println!("The last element of array is {}", arr[4]);
 }
