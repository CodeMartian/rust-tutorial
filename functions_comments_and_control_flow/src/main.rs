fn main() {
    //Hello, World
    println!("Hello, world!");
    another_function(5);
    expression();
    let z = plus_one(five());
    is_less_than_five(z);
    println!("The value of z is: {z}");
    // and_then(); //proceed with caution
    increment();
    wait_a_while();
    iterate();
    element_in_collection();
    number_in_range_reverse();
}

// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it. In reality, I can't think of a 
// reason I would need a comment this long.

fn another_function(x: i32) {
    println!("The value of x is: {x}"); // Comments can also exist at the end of a line.
}

fn expression() {
    //Notice the lack of ; in line 16. This is the return value of the expression.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}")
}

fn five() -> i32 {
    5
}

fn plus_one(int: i32) -> i32 {
    int + 1
}

fn is_less_than_five(number: i32) {
    if number < 5 {
        println!("The condition is true.")
    } else {
        println!("The condition is false.")
    }
}

fn and_then() {
    loop {
        println!("and then... ");
    }
}

fn increment() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter;
        }
    };

    println!("The result of the loop is: {result}");
}

fn wait_a_while() {
    let mut number = 3;

    while number != 0 {
        println!("Wait a while!");
        number -= 1;
    }

    println!("Done waiting.")
}

fn iterate() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("The value of the collection is: {}", a[index]);
        index += 1;
    }
}

fn element_in_collection() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The element is: {element}");
    }
}

fn number_in_range_reverse() {
    for number in (1..4).rev() {
        println!("The reversed number is: {number}");
    }
}