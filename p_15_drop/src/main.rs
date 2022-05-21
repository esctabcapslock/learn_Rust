

fn main() {
    let c = CustomSmartPointer {data:String::from("가나다라")};
    let _d = CustomSmartPointer {data:String::from("마바사아")};
    println!("CustomSmartPointers created");
    println!("dddd {:?}",c); 
    drop(c);
    // drop(c);
    // c.drop(); err: explicit destructor calls not allowed
    // println!("dddd {:?}",c); erR: value borrowed here after move
}

#[derive(Debug)]
struct CustomSmartPointer{
    data:String
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data: {}", self.data)

    }
}