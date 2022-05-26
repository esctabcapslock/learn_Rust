// cd p_19_advanced_lifetimes
// 05.26
// https://rinthel.github.io/rust-lang-book-ko/ch19-02-advanced-lifetimes.html


extern crate p_19_advanced_lifetimes;
use p_19_advanced_lifetimes::{Parser,Context};
fn main() {
    let s = String::from("abcde");

    // let kk = 
    let k = Parser{context:&Context{0:&s}};
    let _k2 = k.parse().expect("hello!!");

    let b = vec![1,2,4,5,6];
    let c = b.iter();
    let d: Vec<i32> = c.map(|b| {b+1}).collect();
    println!("d:{:?}",d);
}