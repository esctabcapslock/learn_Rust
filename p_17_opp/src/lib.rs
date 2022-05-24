// gui 라이브러리 만들기

pub trait Draw{
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw>는 트레잇 객체이다.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // 컴포넌트에 대해 draw 메소드를 호출
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//제네릭 타입 파라미터를 이용해 구현할 경우
//Rust는 상속따위 없기 때문에
//Button이면 Button, Textbox면 Textbox 등 한가지로만 고정된다.

pub struct  Button{
    pub width:u32,
    pub height:u32,
    pub label:String,
}
// impl Draw for Button {
//     fn draw(&Self){

//     }
// }
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("button");
    }
}


pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("selectbox");
    }
}