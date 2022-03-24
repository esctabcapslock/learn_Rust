use std::io::*;
fn readint()->Vec<i32>{
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}

fn main(){
    let t:i32 = readint()[0];
    let data = [
        [10,10,10,10],
        [1,1,1,1],
        [6,2,4,8],
        [1,3,9,7],
        [6,4,6,4],
        [5,5,5,5],
        [6,6,6,6],
        [1,7,9,3],
        [6,8,4,2],
        [1,9,1,9],
        ];
    for _ in 0..t{
        let tmp = readint();
        let (a,b):(usize,usize) = (tmp[0] as usize,tmp[1] as usize);
        println!("{}",data[a%10][b%4]);
    }
}