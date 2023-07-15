#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Colorado,
    Michigan,
    New_Jersey,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let penny_value = value_in_cents(Coin::Penny);
    let nickel_value = value_in_cents(Coin::Nickel);
    let dime_value = value_in_cents(Coin::Dime);
    let quarter_value = value_in_cents(Coin::Quarter(UsState::Colorado));
    println!("The value of a penny is {}", penny_value);
    println!("The value of a nickel is {}", nickel_value);
    println!("The value of a dime is {}", dime_value);
    println!("The value of a quarter is {}", quarter_value);
}
