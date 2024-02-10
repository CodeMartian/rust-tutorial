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
    let _x = 2.0;
    let _y: f32 = 3.0;
}

fn numeric_operations() {
    // addition
    let _sum = 5 + 10;

    //subtraction
    let _difference = 95.5 - 4.3;

    //multiplication
    let _product = 4 * 30;

    //division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // this returns -1

    //remainder
    let _remainder = 43 % 5;
}

fn boolean_values() {
    let _t = true;

    let _f: bool = false; // with explicit type annotation
}

fn char_values() {
    let _c = 'Z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}

fn tuple_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {_y}");

    let new_tup: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = new_tup.0;

    let _six_point_four = new_tup.1;

    let _one = new_tup.2;
}

fn array_types() {
    let a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let _three = [3; 5];

    let _first = a[0];
    let _second = a[1];
}