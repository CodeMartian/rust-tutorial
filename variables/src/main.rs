const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    print_variables();

    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    print_shadow();
    shadow_string_type();
}

fn print_variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

/// .
fn print_shadow() {
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y is: {y}")
    }

    println!("The value of y is: {y}")
}

fn shadow_string_type() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Length of spaces: {spaces}")
}