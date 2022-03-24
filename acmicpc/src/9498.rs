// 이거 왜 안됨...
fn main(){
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    let a:i32 = buf.trim().parse().unwrap();
    let a = a / 10;
    if a==10{println!("A")}
    else if a==9{println!("A")}
    else if a==8{println!("B")}
    else if a==7{println!("C")}
    else if a==6{println!("D")}
    else {println!("F")}
}