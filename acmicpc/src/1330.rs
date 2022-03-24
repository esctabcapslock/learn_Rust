use std::io;
use std::cmp::Ordering;


// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main(){
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    // let splits = io::stdin().split(b'-');
    // for split in splits{
    //     println!("buf print:{}",String::from_utf8_lossy(&split.unwrap()));
    // }    
    let a:Vec<i32> = buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>();
    // match a[0].comp(a[1]){
    //     std::cmp::Ordering::Less: => println!(">");
    // }
    match a[0].cmp(&a[1]){
        Ordering::Less => println!("<"),
        Ordering::Greater => println!(">"),
        Ordering::Equal => println!("=="),
    }
    
    // print_type_of(a[0].cmp(&a[1]));
    
    // println!("{:?},",1);


    // 'g'로 가득한 배열(?)이 10칸 생김
    // let a=['g';10];
    // println!("{:?}",a)
}