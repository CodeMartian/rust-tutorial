struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;
    let tuple_rectangle = (25, 25);
    let struct_rectangle = Rectangle {
        width: 69, 
        height: 420
    };

    println!("Hello, the area of the rectangle is {}!", area(&width, &height));
    println!("Hello, the area of the tuple rectangle is {}!", tuple_area(&tuple_rectangle));
    println!("Hello, the area of the struct rectangle is {}!", struct_area(&struct_rectangle));
}

fn area(width: &u32, height: &u32) -> u32 { // fine
    width * height
}

fn tuple_area(dimensions: &(u32, u32)) -> u32 { // better
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 { // best
    rectangle.width * rectangle.height
}