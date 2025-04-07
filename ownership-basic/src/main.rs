fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {s1}, s2: {s2}"); 

    let s3 = s1;
    // println!("{}", s1); // This will cause a compile-time error because s1 is moved

    let mut _s4 = String::from("world");
    // _s4 goes out of scope here, Rust will run `drop` function
    // on it and its memory will be freed
    _s4 = String::from("hello world");
    println!("s4: {}", _s4);

    let (s5, len) = get_length(_s4);
    println!("The length of {s5} is {len}.");

    // s4 is moved into get_length, so we can't use it here
    let len2 = get_length2(&s3);
    // s3 is passed by reference, so we can still use it here
    println!("The length of {s3} is {len2}.");

    let mut s6 = String::from("hello");
    add_world(&mut s6);
    println!("s6: {}", s6);

    let s7 = &mut s6;
    println!("s7: {}", s7); 
    // println!("s6: {s6}, s7: {s7}"); // cannot borrow `s6` as immutable because it is also borrowed as mutable
    // let s8 = &mut s6; // cannot borrow s6 as mutable more than once

    let mut s9 = String::from("pretty cool");

    let s10 = &s9;
    let s11 = &s9;
    // let s12 = &mut s9; // We cannot borrow s9 as mutable while it is borrowed as immutable

    // println!("s9: {s9}, s10: {s10}, s11: {s11}"); // This is fine because s9 is not borrowed as mutable4
    println!("s10: {s10}, s11: {s11}"); 
    // s10 and s11 will not be used after this point

    let s12 = &mut s9;
    println!("s12: {s12}");

    let mut s13 = String::from("pretty good");
    // s13.clear(); // Works fine because s13 is not borrowed
    let word_len = first_word(&s13);
    s13.clear(); 
    println!("word_len: {word_len}");

    s13.push_str("pretty bad");
    let word = first_word2(&s13);
    // s13.clear(); // error: cannot borrow `s13` as mutable because it is also borrowed as immutable `word`
    println!("word: {word}");

    let i1 = [1, 2, 3, 4, 5];
    let slice = &i1[0..2]; 
    assert_eq!(slice, &[1, 2]); 
}

fn get_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn get_length2(s: &String) -> usize {
    s.len()
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // return index
        }
    }

    s.len() // return length
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}