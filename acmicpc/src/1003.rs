use std::io::*;
fn main(){
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    let n:i32 = buf.trim().parse().expect("잘못된 수임");
    // let mut p:usize = 0;
    // p += 1;

    for _i in 0..n{  
        // 이걸 새로 정의하지 않으면, 입력이 '이어져서' 들어온다...
        // 제곱: n.pow(2)
        // 비교: n.comp(&m)
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read line");
        let a:Vec<i32> = buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>();
        // println!("ed dis:  bot {:?}",a);
        let dis:i32 = (a[0]-a[3])*(a[0]-a[3]) + (a[1]-a[4])*(a[1]-a[4]);
        let (r1, r2):(i32,i32) = (a[2], a[5]);
        let radd = (r1+r2)*(r1+r2);
        // println!("ed dis // rr: {} {}",dis, radd);

        if dis==0{//같은점
            if radd == 0{ //둘 다 0
                println!("0");
            }else if r1 == r2{//같은데 0 아님
                println!("-1");
            }else{
                println!("0");
            }
        }else{
            if radd < dis{
                println!("0");
            }else if radd==dis{
                println!("1");
            }else if dis > (r2-r1)*(r2-r1){
                println!("2");
            }else if dis == (r2-r1)*(r2-r1){
                println!("1");
            }
            else{
                println!("0");
            }
        }
        // =6;
    }
    // let splits = io::stdin().split(b'-');
    // for split in splits{
    //     println!("buf print:{}",String::from_utf8_lossy(&split.unwrap()));
    // }    
    
    
    // println!("{:?}",a[0]-a[1]);


    // 'g'로 가득한 배열(?)이 10칸 생김
    // let a=['g';10];
    // println!("{:?}",a)
}