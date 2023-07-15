pub mod garden;
use crate::garden::vegetables::Vegetables;


fn main() {
    let carrot = Vegetables::Carrot;
    println!("Hello, world! This is a {:?}", carrot);
}
