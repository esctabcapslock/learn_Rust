// #[test]/
// cargo test
struct myBox<T>(T);
impl <T> myBox<T>{
    fn new(x:T) -> myBox<T>{
        myBox(x)
    }
}

fn main() {
    let x = 5;
    let y = myBox::new(x);

    // *는 역참조 연산자
    // Box<T>를 이용헤 역참조 연산자 구현
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

