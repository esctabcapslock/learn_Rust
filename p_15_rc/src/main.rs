use std::rc::Rc;

enum List{Conss(i32,Rc<List>),Nil,}
use List::{Conss, Nil};

fn main() {
    println!("Hello, world!");
    let a = Rc::new(Conss(5, Rc::new(Conss(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Conss(3, Rc::clone(&a)); //1
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let _c = Conss(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); //3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); //2
    // weak_count가 어딘가에 있데   

}
