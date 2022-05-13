fn readline()->(i32,i32){
    println!("12");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    let tmp:Vec<&str> = buf.split_whitespace().collect();
    println!("tmp {:?}",tmp);
    let mut kk = 0;
    println!("{}",kk);
    match tmp[0].chars().nth(0){
         Some('*') => kk=1,
         Some('/') => kk=2,
         Some('^') => kk=3,
         _ => kk=0,
     }
    (kk,tmp[1].parse().expect("err"))
}

fn readint()->Vec<i32>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}

fn main(){
    // ^ 연산자: 지들끼리만 한 점 (0,1)만 만남
    // 다른 놈들과는 무조건(y=0 빼고) 1회 만남

    // * 연산자, 지들끼리만 원점    에서 만남. 
    // a=0이면, /와 안만남.
    //  / 연산자와는 a의 부호가 같으면 2번, 아니면 0번만남

    // / 연산자, 축 위 못감
    // 지들끼리 못 만남

    // 세 저머 만날 수 있는곳

    // eax = e/b/x -> x=+-1/sqrt(ab), y=+-e sqrt(a/b)

    // 즉,e^(+=c/sqrt(ab))가 정수여 함. 
    //  e^( c/sqrt(a*b)) = e sqrt(a/b)
    // a==b만 가능하며, e = e^(c/a)므로 , a==b==c일때만 세 점 다 만남

    // ex. y=e^x, y=ex, y=e/x
    let n = readint()[0];
    // return;
    // let _arpw = [[0 as i32;100000]; 2];
    let _arpw = vec![0 as i16;300000];
    let _armt = vec![0 as i16;300000];
    let _ardv = vec![0 as i16;300000];
    // return;
    let _top_pw = 0;
    let _top_mt = 0;
    let _top_dv = 0;
    for _ in 0..n{
        let ar = readline();
        if ar.0 == 1{
            println!("fe");
        }
    }

    
}