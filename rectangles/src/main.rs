#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // best
    fn area(&self) -> u32 { // This is a method.. notice the self.
        self.width * self.height
    }

    fn can_fit_within(&self, other_rectangle: &Rectangle) -> bool { // Can include other parameters
        self.width <= other_rectangle.width && self.height <= other_rectangle.height
    }

    fn square(size: u32) -> Self { // Associated function
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;
    let tuple_rectangle = (25, 25);
    let struct_rectangle = Rectangle {
        width: 69, 
        height: 420
    };
    let first_rectangle = Rectangle {
        width,
        height
    };
    let second_rectangle = Rectangle {
        width: tuple_rectangle.0,
        height: tuple_rectangle.1
    };

    let square = Rectangle::square(30);

    println!("Hello, the area of the rectangle is {}!", area(&width, &height));
    println!("Hello, the area of the tuple rectangle is {}!", tuple_area(&tuple_rectangle));
    println!("Hello, the area of the struct rectangle is {}!", struct_area(&struct_rectangle));
    println!("Hello, the area of the struct rectangle based on its own calculation is {}!", &struct_rectangle.area());
    println!("The first rectangle can fit in the second rectangle {}", first_rectangle.can_fit_within(&second_rectangle));
    println!("The second rectangle can fit in the struct rectangle {}", second_rectangle.can_fit_within(&struct_rectangle));
    println!("The struct rectangle can fit in the first rectangle {}", struct_rectangle.can_fit_within(&first_rectangle));
    println!("The size of the square is width {}, height {}", &square.width, &square.height);
}

fn area(width: &u32, height: &u32) -> u32 { // fine
    width * height
}

fn tuple_area(dimensions: &(u32, u32)) -> u32 { // better
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 { // even better
    rectangle.width * rectangle.height
}