// #[warn(non_snake_case)]
use std::rc::{Weak,Rc};
use std::cell::{RefCell};
use List::{Cons, Nil};

#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List{
    fn tail(&self)  -> Option<&RefCell<Rc<List>>> {
        match *self{
            Cons(_,ref item) => Some(item),
            Nil => None,
        }
    }
}

fn cycle(){
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
        // let c = *link;
    }
    // let Some(c)= a.tail();
    // c;

    // a.tail().borrow_mut() = Rc::clone(&b);

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}



// # 트리 데이터 구조 만들기

#[derive(Debug)]
// #[warn(dead_code)]
struct Node{
    value:i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn tree(){
    let leaf = Rc::new(Node{
        value:3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node{
        value:5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new())
    });

    *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let k = &*branch.children.borrow();

    println!("cbranch.hildren: {:?}, value:{}",k, branch.value);

    // branch
}


fn main(){
    cycle();
    tree();
}

