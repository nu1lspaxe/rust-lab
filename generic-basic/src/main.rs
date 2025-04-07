fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = find_largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let int_float = Point { x: 1, y: 4.0 }; // This will not compile because x and y have different types

    let int_float = Point2 { x: 1, y: 4.0 };

    println!("Integer Point: x = {}, y = {}", integer.x(), integer.y());

    let p3 = Point3 { x: 5, y: 10.5 };
    let p4 = Point3 { x: "hello", y: 'c' };
    let p4 = p3.mixup(p4);
    println!("Mixed Point: x = {}, y = {}", p4.x, p4.y);

    let s1 = String::from("long string");
    {
        let s2 = String::from("short");
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {}", result);
    }
}

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}


impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

use std::fmt::Display;

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

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where 
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}