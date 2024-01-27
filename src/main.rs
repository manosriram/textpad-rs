mod textfile;
mod textpad;
use std::{borrow::Borrow, collections::HashMap};

// use crate::textfile::textfile::FileRow;

fn main() {
    println!("Hello, world!");

    let mut h: HashMap<i32, i32> = HashMap::new();

    h.entry(1).or_insert(100);
    *h.entry(1).or_insert(300) += 100;

    let mut s: String = String::from("hi");
    s += &String::from(" world").borrow();
    println!("{} {}", s, s.clone());

    let op: Option<&i32> = h.get(&1);
    match op {
        Some(x) => {
            println!(" {} ", x);
        }

        None => {
            println!("got none");
        }
    }
}
