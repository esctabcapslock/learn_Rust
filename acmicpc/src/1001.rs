use std::io;
fn main(){
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    // let splits = io::stdin().split(b'-');
    // for split in splits{
    //     println!("buf print:{}",String::from_utf8_lossy(&split.unwrap()));
    // }    
    let a:Vec<i32> = buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>();
    
    
    println!("{:?}",a[0]-a[1]);


    // 'g'로 가득한 배열(?)이 10칸 생김
    // let a=['g';10];
    // println!("{:?}",a)
}