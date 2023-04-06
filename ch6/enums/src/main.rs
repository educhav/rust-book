enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}


// How the standard library implements IpAddr - takes in a struct as an associated value for enum
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


// Wide variety of types embedded in the variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
    }
}


fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let message = Message::Move {
        x: 1,
        y: 2
    };

    let some_number = Some(5);
    let some_char = Some('e');

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = Some(7);

    let absent_number: Option<i32> = None;
}
