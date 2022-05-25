pub struct Post{
    
    // 이전 상태를 소비하기 위해서 request_review 메소드는 상태 값의 소유권을 취할 필요가 있습니다. 
    // 이것이 Post의 state 필드 내 Option이 들어온 까닭입니다
    // 우리는 take 메소드를 호출하여 state 필드 밖으로 Some 값을 빼내고 그 자리에 None을 남기는데, 
    // 왜냐면 러스트는 구조체 내에 값이 없는 필드를 허용하지 않기 때문입니다.  -> 이 줄이 핵심인듯?? 그렇데. 그래서 copy를 한거군...
    // 값을 이동하니까...
    // 이는 우리가 state 값을 빌리기 보다는 게시물 밖으로 이동시키도록 만듭니다. 
    // 이후 우리는 게시물의 state 값을 이런 연산의 결과물로 설정하려고 합니다.
    // 뭔소리임
    
    state:Option<Box<dyn State>>,
    // state:Box<dyn State>,
    content:String
    //https://doc.rust-lang.org/std/boxed/struct.Box.html
    //box는 전역 힙에 설정하는 것이다.
    //일종의 포인터 타입이래
}
impl Post {
    pub fn new() -> Post{
        Post {
            
            state: Some(Box::new(Draft {})),
            // state: Box::new(Draft {}),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // ""
        let k = self.state.as_ref().unwrap();
        k.content(&self)
        // self.state.content(&self)
    }

    //게시물의 리뷰를 요청하는 기능임
    // state 객체를 PendingReview 이후로 바꿈
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            // s:Box<dyn State>
            self.state = Some(s.request_review())
        }
        // self의 소유권을 갖고 있지 않기 때문 아닐까?
        // let k =self;
        // let kk = self.state.try_into().unwrap();
        
        // option 없이 self.state.request_review(); 하면
        // 다중 참조자가 생겨서 망한다.
        // let k = self.state.request_review();
        // self.state = k
            //cannot move out of `self.state` which is behind a mutable reference
            //move occurs because `self.state` has type `Box<dyn State>`, which does not implement the `Copy` trait
        // let k = self.state.as_mut();
        // let kk =  Box::new(*k);
        //the size for values of type `dyn State` cannot be known at compilation time
        // the trait `Sized` is not implemented for `dyn State` 


        // self.state = Box::new(Draft{})
        // let k = Option::Some(Box::new(Draft{}));
        // k.copy();
        // {
            // 다음 함수를 통해 self.state가 반환됨.
            
            // self.state.request_review();
        // }

        // self.state = k
        // if let Some(s) = Option::Some(self.state).take(){
        //     self.state = s.request_review()
        // }
 
        // let kk = self.state.as_ref();

        // let kkk = kk.request_review();

        // self.state = kkk;
    }
    // 승인하다.

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }

        
        // let k = self.state.approve();
        // self.state = k;
        
        // let kk = self.state.as_mut().approve();
        // self.state = kk;

    }

}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // 여기에 Post를 넣야 하는게 뭔가 내 방식과 다른듯.
    // 여러 클래스에 의존하는 느낌...?
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}


struct Draft{
}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    //
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct PendingReview{

}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        // self
        Box::new(Published {})  
    }
}


struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}