//cd p_17_opp
// https://rinthel.github.io/rust-lang-book-ko/ch17-02-trait-objects.html
extern crate p_17_opp;
use p_17_opp::{Screen,Button,SelectBox};

fn main() {
    // duck typing 개념느낌
    // 행동이 같으면 묶을 수 있음
    println!("Hello, world!");
    let screen  = Screen{ components: vec![
        Box::new(
            SelectBox{
                width:10,
                height:100,
                options:vec![
                    String::from("hell1"),
                    String::from("world"),
                ],
            }
        ),
    Box::new( Button{
            width:10,
            height:23,
            label:String::from("input"),
        }
    )], };
    screen.run();
    // print!({:?},);

}
