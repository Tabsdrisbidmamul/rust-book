enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self)
    }
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home {:#?} \nloopback {:#?}", home, loopback);

    let m = Message::Write(String::from("test"));
    m.call();

    let some_number = Some::<i32>(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("some_number is_some {}", some_number.is_some());
    println!("some_char is_some {}", some_char.is_some());
    println!("absent_number is_none {}", absent_number.is_none());
    println!("absent_number is_some {}", absent_number.is_some());

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

// fn route(ip_kind: IpAddrKind) {}
