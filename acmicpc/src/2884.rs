fn readint()->Vec<i32>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}
fn main(){
    let tmp=readint();
    let (mut h,mut m) = (tmp[0],tmp[1]);
    let c=readint()[0];
    m += c%60;
    h += c/60;
    if m>=60 {h+=m/60; m%=60;}
    h %= 24;
    
    println!("{} {}",h,m);
}