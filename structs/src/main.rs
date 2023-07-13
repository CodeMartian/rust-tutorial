#[derive(Debug)]
struct User { // This is a struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32,
}

#[derive(Debug)]
struct Color(i32, i32, i32); // These are tuple structs 
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("CodeMartian"),
        email: String::from("jeff@codemartian.com"),
        sign_in_count: 0,
    };

    user1.email = String::from("knowledgeispower@codemartian.com");

    println!("{:?}", &user1);

    let user2 = build_user(String::from("jeffmmartin@codemartian.com"), String::from("mr_wubwubs"));
    
    println!("{:?}", &user2);

    let user3 = User {
        email: String::from("userfromanothamotha@codemartian.com"),
        ..user1
    };

    println!("{:?}", &user3);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("{:?}", black);
    println!("{:?}", origin);

    let equal = AlwaysEqual;

    println!("{:?}", equal)
    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 0,
    }
}
