fn main() {
    let words = String::from("Hello, world!");
    let first_word = first_word(&words);
    println!("The first word in words is {}", &first_word);
    let array = [1,2,3,4,5];
    let array_slice = other_slice(&array);
    println!("The elements in the first 3 positions in the array are {:?}", &array_slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn other_slice(arr: &[i32]) -> &[i32] {
    &arr[..=2]
}