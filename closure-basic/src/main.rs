use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "User preference: {:?}, Giveaway: {:?}", 
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "User preference: {:?}, Giveaway: {:?}", 
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    let list = vec![1, 2, 3];
    
    let only_borrows = || println!("Borrowed: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list2 = vec![1, 2, 3];
    println!("Before calling closure: {list2:?}");

    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("After calling closure: {list2:?}");

    thread::spawn(move || println!("Thread spawned with list: {list:?}"))
        .join()
        .unwrap();

    let mut list3 = [
        Rectangle { width: 30, height: 50 },
        Rectangle { width: 10, height: 40 },
        Rectangle { width: 20, height: 60 },
    ];

    let mut num_sort_operations = 0;
    list3.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list3:#?}, sorted in {num_sort_operations} operations");

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}