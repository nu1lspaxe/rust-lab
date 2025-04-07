enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

struct Message2 {
    quit: QuitMessage,
    move_msg: MoveMessage,
    write_msg: WriteMessage,
    change_color_msg: ChangeColorMessage,
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, b: {}", r, g, b),
        }
    }
}

fn main() {
    let _home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };
    
    let _loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    
    let m = Message::Write(String::from("hello"));
    m.call();

    let dice_roll = 9;
    let value = match dice_roll {
        3 => "You rolled a three!",
        7 => "You rolled a seven!",
        9 => "You rolled a nine!",
        _ => "You rolled something else!",
    };
    println!("{}", value);

    // `if let` equals `match`
    if let Message::Write(text) = m {
        println!("The message is: {}", text);
    } else {
        println!("The message is not a write message.");
    }
}

#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}