fn readint()->Vec<i32>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}

fn main(){
    let (a,b) = (readint()[0], readint()[0]);
    print!("{}\n{}\n{}\n{}",a*(b%10), a*((b/10)%10), a*((b/100)%10), a*b)
}
