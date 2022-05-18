// #[test]/
// cargo test
struct MyBox<T>(T);
impl <T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }

    // fn deref(&self) -> &T{
    //     &self.0
    // }
}

// dereference 의 앞부분을 따서 deref
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    // *는 역참조 연산자. 참조자가 뭘 참조하고 있는지 할 수 있다.
    // Box<T>를 이용헤 역참조 연산자 구현
    assert_eq!(5, x);
    // Deref 트레잇 없이, 컴파일러는 오직 & 참조자들만 역참조 할 수 있다.
    // rust는 *y 대신 *(y.deref())를 실행한다.
    assert_eq!(5, *y);

    // #  암묵적 역함수 강제
    let m = MyBox::new(String::from("Rust"));
    
    // let mm = *m;
    // let mm = m.deref();
    // 위의 것들은 정상적으로 실행됨
    //hello(&***m); // 에러: type `str` cannot be dereferenced
    //hello(*m); 에러: expected `&str`, found struct `String`

    // cf. 스크링 리터럴은 슬라이드

    // m: MyBox<String>
    // &m: &MyBox<String>
    // m.deref(): &String
    // *m: String
    // **m: str
    // ***m: uknowm
    // &*m: &String
    // *&*m: String
    // &*&*m: &String
    

    hello(&*&&**&m);
    hello(&*&&**m);
    hello(&*&*m);
    hello(&**m);
    hello(&*m);
    hello(m.deref());
    hello(&m); //분명 넣은건 &MyBox<String> 이런건데 &str이 인자인 함수에 들어가짐
    hello(&String::from("3425"));
    let _k = &String::from("3425")[..];
    // Deref 트레잇이 관련된 타입에 대해 정의될 때, 러스트는 해당 타입을 분석하여 파라미터의 타입에 맞는 참조자를 얻기 위해 필요한 수만큼의 Deref::deref를 사용
    
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}

