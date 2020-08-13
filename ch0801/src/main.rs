// Storing Lists of Values with Vectors

fn main() {
    let v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v1[2];
    println!("The third element is {}", third);
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{}", i)
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        println!("element before mut:{}", i);
        *i += 50;
        println!("element after mut:{}", i)
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}