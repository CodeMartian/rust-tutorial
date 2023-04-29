fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of {} is {} characters", s1, len); // Since the reference of s1 is used here, it frees up for another reference (mutable or not) 

    change(&mut s1); // Mutable reference to s1

    let len = calculate_length(&s1); // Immutable reference to s1 here
    println!("The length of {} is now {} characters", s1, len); // Both references are cleaned up here
}

fn calculate_length(string: &String) -> usize {
    string.len()
}

fn change(string: &mut String) {
    string.push_str(", world")
}