
/***
*
* 1) --- create
*
* 2) --- reference
* When the program has a valid reference, the borrow checker enforces the ownership and borrowing
* rules (covered in Chapter 4) to ensure this reference and any other references to the contents
* of the vector remain valid.
*
* 3) --- loop
*
* 4) --- using enum to store multiple types
*
***/
pub fn demo() {
    println!("###### <<< Rust vectors ######");
    demo_create();
    demo_loop();
    demo_mul_types();
    println!("###### Rust vectors >>> ######");
    println!("");
}

// --- 1) -----------
fn demo_create() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(10);
    v1.push(11);
    v1.push(12);

    let third: &i32 = &v1[2];
    println!("The third element of v1 is {}", third);

    // -- THIS CODE WON'T WORK because of 2) --
    // v1.push(13);
}

// --- 3) ----
fn demo_loop() {
    let mut v = vec![1,2,3];
    for i in &mut v {
        *i += 1;
    }

    println!("After plus 1, the first element of v is {}", &v[0]);
}

// --- 4) ---
#[derive(Debug)]
enum Cell {
    Int(i32),
    Text(String),
}

fn demo_mul_types() {
    let cells = vec![Cell::Int(3), Cell::Text("Three".to_string())];
    println!("The first element of cells is {:?}", &cells[0]);
    println!("The second element of cells is {:?}", &cells[1]);
}
