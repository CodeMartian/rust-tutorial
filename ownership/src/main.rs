use std::ops::Neg;

fn main() {
    let s = String::from("hello"); //s scope begins

    takes_ownership(s); //s value moves into the function, and is no longer valid. This is because it is stored in heap.

    let x = 32;

    makes_copy(x);

     let y = x.neg();
    println!("{}", y);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}