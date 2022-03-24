fn main(){
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    let a:i32 = buf.trim().parse().unwrap();
    println!("{}", (if a%400!=0 && (a%4!=0 || a%100==0) {0}else{1}));
}