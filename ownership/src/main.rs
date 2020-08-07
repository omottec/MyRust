fn main() {
    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);


    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}", s1);
    println!("s2: {}", s2);


    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);


    let s = String::from("hello world");
    let x = 10;
    takes_ownership(s);
    makes_copy(x);
    // println!("s:{}, x:{}", s, x);
    println!("x:{}", x);


    let s5 = gives_ownership();
    let s6 = String::from("bbb");
    let s7 = takes_and_gives_back(s6);
    println!("s5:{}", s5);
    // println!("s6:{}", s6);
    println!("s7:{}", s7);

    let s8 = String::from("s8");
    let (s9, length) = calculate_length(s8);
    // println!("s8:{}", s8);
    println!("s9:{}, length:{}", s9, length);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn gives_ownership() -> String {
    let some_string = String::from("aaa");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
