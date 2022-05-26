pub struct Context<'a>(pub &'a str); //expected named lifetime parameter

pub struct Parser<'b,'bb> {
    pub context: &'b Context<'bb>,
}

impl<'c, 'cc> Parser<'c, 'cc> {
    pub fn parse(&self) -> Result<(), &'cc str> {
        Err(&self.context.0[1..])
    }
}

pub fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()

    // cannot return value referencing function parameter `context`
    // returns a value referencing data owned by the current function
    // 우리는 Context 내의 스트링 슬라이스와 Parser 내의 Context를 가리키는 참조자가 다른 라이프타임을 가지고 있고
    // parse_context의 반환값은 Context의 스트링 슬라이스의 라이프타임에 묶여있음을 알려줄 방법이 필요합니다.

    //문법 'b: 'a를 사용하여 'b를 선언함으로써 라이프타임 'b가 최소 'a 만큼 오래 산다고 선언할 수 있습니다.
    // 안 해도 컴파일되는데?? 어이어이
}

// ## 제네릭 타입에 대한 참조자 상의 라이프타임 바운드
#[warn(dead_code)]
struct Ref<'a, T>(&'a T); // 책은 오류난다고 하는데 음 오류 안나는데 뭐지

impl<'a, T> Ref<'a, T> {
    
}