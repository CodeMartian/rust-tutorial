fn main() {
    println!("Hello, world!");

    floating_point_types();
    numeric_operations();
    boolean_values();
    char_values();
    tuple_types();
    array_types();
}

fn floating_point_types() {
    let x = 2.0;
    let y: f32 = 3.0;
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // this returns -1

    //remainder
    let remainder = 43 % 5;
}

fn boolean_values() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

fn char_values() {
    let c = 'Z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn tuple_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let new_tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = new_tup.0;

    let six_point_four = new_tup.1;

    let one = new_tup.2;
}

fn array_types() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let three = [3; 5];

    let first = a[0];
    let second = a[1];
}