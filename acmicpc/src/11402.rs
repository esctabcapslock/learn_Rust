    // 윌슨정리
    fn readint()->Vec<i64>{
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Failed to read line");
        buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
    }

    fn fact(n:i64, p:i64)->[i64;2]{
        // 준식이 A * (B**P)면 [A,B] 반환. 
        // ex. 3!, 3 -> [2,1]
        // ex. 3!, 3 -> [2,1]
        if n==0 {return [1,0];}
        if n<p{ 
            let mut out = 1;
            for i in 1..(n+1) {out = (out*i)%p;}
            // println!("fact - el n{} p{} {}",n,p,out);
            return [out,0];
        }else {
            // let mut pp = p;
            let k = n/p;
            let tmp = fact(k,p);
            // let mut out = k;
            // while k!=0{
            //     pp *= p;
            //     k = p/pp;
            //     out += k;
            // }
            // let 
            // println!("fact n{} p{} {:?} {}",n,p,fact(n%p,p),(p+ (if n/p%2==0 {1 }else {-1}))%p);
            return [(p+fact(n%p,p)[0]*(if n/p%2==0 {1 }else {p-1})*tmp[0])%p,tmp[1]+k];
        }
    }

    fn main(){
        let tmp = readint();
        let (n,k,p) = (tmp[0],tmp[1],tmp[2]);
        let (a,b,c) = (fact(n,p), fact(k,p), fact(n-k,p));

        // println!("{:?}, {:?}, {:?}",a,b,c);
        if a[1]-b[1]-c[1] > 0{
            println!("0");
        }else{
            let (x,y) = (a[0], (b[0]*c[0])%p);
            for i in 0..p{
                if (x + p*i)%y == 0{
                    println!("{}",(x + p*i)/y);
                    break;
                }
            }
        }

        // N!을 구해보자. M의 들은 p의 개수는 [n/p] + [n/p^2] + ... 이 꼴이다.
        // 1 ~ p-1 까지가 -1이므로, (-1).pow(n/p) 곱한다.
        // N%p 일 경우, 이를 반복해서 구한다.
        
    }

    // mod 7의 경우
    // 2*4 == 1
    // 6*6 == 1
    // 3*5 == 1
    // (2*6)^-1 == (5)^^-1 == 3
    // 4*6 == 24 == 3

    // 100 40 17 -> -15