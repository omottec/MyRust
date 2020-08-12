// Concise Control Flow with if let

fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three by match"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three by if let");
    }
    println!("Hello, world!");
}
