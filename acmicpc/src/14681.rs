fn readint()->Vec<i32>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}
fn main(){
    let x=readint()[0];
    let y=readint()[0];
    println!("{}",(if x>0 && y>0 {1} else if x<0 && y>0 {2} else if x<0 && y<0 {3} else {4}))
}