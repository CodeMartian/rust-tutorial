use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1));

    println!("Hello, world! \n{:#?}, \n{:#?}", home, loopback);

    let some_thing = Some(5);
    let some_char = Some('e');

    let absent_thing: Option<u32> = None;

    println!("Here's something: {:?}", some_thing);
    println!("Here's something: {:?}", some_char);
    println!("Here's nothing: {:?}", absent_thing);
}
