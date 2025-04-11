use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak}; // Rc<T>, the reference counted smart pointer

fn main() {
    let x = 5;
    // let y = &x;
    let y = Box::new(x);
    
    // let list = Cons(
    //     1, Box::new(
    //         Cons(
    //             2, Box::new(
    //                 Cons(3, Box::new(Nil))
    //             )
    //         )
    //     )
    // );

    let a = 5;
    let b = MyBox::new(5);
    assert_eq!(5, *y);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c);
    println!("CustomSmartPointer created.");

    // let rc_list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    // println!("Reference count: {}", Rc::strong_count(&rc_list));
    // let rc_list2 = Cons(1, Rc::clone(&rc_list));
    // println!("Reference count: {}", Rc::strong_count(&rc_list));
    // let rc_list3 = Cons(2, Rc::clone(&rc_list));
    // {
    //     let rc_list4 = Cons(3, Rc::clone(&rc_list));
    //     println!("Reference count: {}", Rc::strong_count(&rc_list));
    // }
    // println!("Reference count: {}", Rc::strong_count(&rc_list));

    let value = Rc::new(RefCell::new(5));
    let x = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let y = Cons(Rc::new(RefCell::new(6)), Rc::clone(&x));
    let z = Cons(Rc::new(RefCell::new(7)), Rc::clone(&x));

    *value.borrow_mut() += 10;
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);

    let leaf = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 10,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    )
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}