fn main() {
    println!("Hello World!");
    let three = 3;
    add_two(&three);
}

fn add_two(x: &u32) -> u32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use crate::add_two;

    #[test]
    fn passes() {
        assert!(true);
    }

    #[test]
    fn fails() {
        assert!(false);
    }

    #[test]
    #[ignore]
    fn ignored() {
        assert!(true);
    }

    #[test]
    fn add_two_returns_5_given_3() {
        let three = 3;
        assert_eq!(add_two(&three), 5)
    }
}