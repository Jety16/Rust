enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) -> u8 {
        match &self {
           Message::Quit => 0,
           Message::Move{x:_, y: _}=> 1,
           Message::Write(_) => 2,
           Message::ChangeColor(_, _, _) => 3,
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let home = IpAddr::V4(String::from("0.0.0.0"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("el_pepe"));
    m.call();

}
