fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1:{}, s2:{}", s1, s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s:{}", s);

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4;
    // println!("s3:{}", s3);
    println!("s4:{}, s5:{}", s4, s5);

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");
    let s9 = format!("{}-{}-{}", s6, s7, s8);
    let s10 = s6 + "-" + &s7 + "-" + &s8;
    println!("s9:{}, s10:{}", s9, s10);

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    println!("hello length:{}", hello.len());

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
