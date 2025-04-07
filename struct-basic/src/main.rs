struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}


fn main() {
    let _user1 = build_user(
        String::from("Bc8bK@example.com"),
        String::from("Bc8bK"),
    );

    let scale = 2;
    let rect1 = Rectangle2 {
        // dbg! returns ownership of the expression's value
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);



fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// Tuple Structs
// Try to use tuple structs when possible because they're 
// readable and self-documenting.
fn area2(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Adding useful functionality with derived traits

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }

    // let sq = Rectangle2::square(30);
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

