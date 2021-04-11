fn main() {
    let ipv4 = IpAddrKind::V4 (127, 0, 0, 1);
    let ipv6 = IpAddrKind::V6 ("::1".to_string());
    println! ("{:?} {:?}", ipv4, ipv6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from ("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from ("::1"),
    // };

    // println! ("{:?} {:?}", home, loopback);

    let m = DifferentStructs::Write(String::from ("Hello, world!"));
    m.call ();
    outside_call (&m);

    // std::Option enum : protects against common null mistakes
    let o : Option<String> = Some(String::from ("Hello world"));
    println! ("{:?}", o);

    println! ("{}", value_int_cents(Coin::Penny));
    println! ("{}", value_int_cents(Coin::Quarter(UsState::California)));

    let five = Some(5);
    println! ("{:?}", five);
    let six = plus_one(five);
    println! ("{:?}", six);
    let none = plus_one(None);
    println! ("{:?}", none);

    // matches are _exhaustive_ by default but Rust provides `_` as a catch-all arm
    let some_u8_val = 3u8;
    match some_u8_val {
        1 => println! ("one"),
        3 => println! ("three"),
        5 => println! ("five"),
        7 => println! ("seven"),
        _ => (),
    }
}

#[derive(Debug)]
enum DifferentStructs {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor (i32, i32, i32),
}

fn outside_call(d: &DifferentStructs) {
    println! ("call invoked: {:?}", d);
}

impl DifferentStructs {
    fn call (&self) {
        println! ("call invoked: {:?}", self);
    }
}

// ^ equivalent to:

struct Quit;
struct Move {
    x: i32,
    y: i32,
}
struct Write (String);
struct ChangeColor (i32, i32, i32);

#[derive(Debug)]
enum IpAddrKind {
    V4 (u8, u8, u8, u8),
    V6 (String),
}

// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// enum IpAddrKind {
//     V4 (String),
//     V6 (String),
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// match

#[derive(Debug)]
enum UsState {
    California,
    NewYork,
    Colorado,
    // etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // pattern that binds to value
    Quarter (UsState),
}

fn value_int_cents (c : Coin) -> u8 {
    match c {
        Coin::Penny => {
            println! ("penny, woo!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter (state) => {
            println! ("State quarter from {:?}!", state);
            25
        },
    }
}

// matching with Option<T>

fn plus_one(o: Option<i32>) -> Option<i32> {
    match o {
        None => None,
        Some(i) => Some(i + 1),
    }
}