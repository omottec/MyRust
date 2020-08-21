use std::fmt::Display;

struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    // A method that uses different generic types from its struct's definition
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest1<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p1 = Point1 { x:5 , y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    let result1 = largest1(&number_list);
    println!("The result is {}", result);
    println!("The result1 is {}", result1);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    let result1 = largest1(&number_list);
    println!("The result is {}", result);
    println!("The result1 is {}", result1);
}
