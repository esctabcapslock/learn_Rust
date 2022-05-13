fn readint()->Vec<i32>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}
fn main(){
    let tmp=readint();
    let (a,b,c) = (tmp[0],tmp[1],tmp[2]);

    if a==b && b==c{
        println!("{}",10000+a*1000);
    }else if a==b || b==c {
        println!("{}",1000+b*100);
    }else if c==a{
        println!("{}",1000+c*100);
    }else{
        println!("{}",a.max(b).max(c)*100);

    }
}