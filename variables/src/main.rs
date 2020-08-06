const MAX_POINTS: u32 = 100_000;

fn main() {
    // mut variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
}
