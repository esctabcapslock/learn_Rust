use std::io::*;

fn readint()->Vec<i32>{
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}

fn main(){
    let t:i32 = readint()[0];
    for _ in 0..t{
        let tmp = readint();
        let (n,k) = (tmp[0], tmp[1]);
        let ds = readint();
        for _ in 0..k{
            let tmp = readint();
            let (cx,cy,r) = (tmp[0],tmp[1],tmp[2]);

            let f = incircle(x1,y1,cx,cy,r)^incircle(x2,y2,cx,cy,r);
            sum += f as i32;

        }


        println!("{}",sum);
    }
}