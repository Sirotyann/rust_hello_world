mod basic;
mod collectors;
mod feature;
mod module_example;

fn main() {
    module_example::demo();
    basic::variables::demo();
    basic::types::demo();
    basic::flow::demo();
    basic::ownership::demo();
    basic::strings::demo();
    basic::structs::demo();
    basic::enums::demo();
    basic::functions::demo();
    basic::traits::demo();
    collectors::vectors::demo();
    collectors::strings::demo();
    collectors::maps::demo();
    basic::pointers::demo();
    feature::cons_list::demo();
}
