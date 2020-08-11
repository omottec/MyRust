// Defining an Enum
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddress {
    V4(String),
    V6(String),
}

enum IpAddress1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit, // has no data
    Move { x: i32, y: i32 }, // includes an anonymous struct
    Write(String),  // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

impl Message {
    fn call(&self) {
        // code
    }
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    println!("Hello, world!");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
}
