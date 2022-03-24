// 풀이 찾아봄... https://seokjin2.tistory.com/13
// 이거 어케 생각함. ㄷㄷㄷㄷ

// A^n 구하는거 그냥 구하면 되는구나. 
// [ a, b
//   c, d  ]
// 


fn readint()->Vec<i64>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}

#[derive(Debug)]
struct Mt{
    a:i64,
    b:i64,
    c:i64,
    d:i64,
}
impl Mt{
    fn new(a:i64, b:i64, c:i64, d:i64)->Mt{
        Mt{a,b,c,d}
    }
    fn mt(&self, m:&Mt)->Mt{
        Mt{
            a:(self.a*m.a + self.b*m.c)%1000, 
            b:(self.a*m.b + self.b*m.d)%1000,
            c:(self.c*m.a + self.d*m.c)%1000,
            d:(self.c*m.b + self.d*m.d)%1000
        }
    }
    // fn get(&self)->i64{
    //     ((2.23606797749979*(self.b as f64) + (self.a as f64)) as i64)%1000
    // }
}
impl Copy for Mt {}
impl Clone for Mt {
    fn clone(&self) -> Self {
        Mt {a:self.a, b:self.b, c:self.c, d:self.d}
    }
}



fn main(){
    let t:i64 = readint()[0];
    let mut ar = vec![Mt::new(6,-4,1,0);32];
    // ar[0] = Mt::new(1,0,0,1);
    for i in 1..31{
        ar[i] = ar[i-1].mt(&ar[i-1]);
        // println!("{}답:{:#?}",i,ar[i]);
    }

    for _t in 0..t{
        let n:i64 = readint()[0]-1;
        let mut i=0;
        let mut out = Mt::new(1,0,0,1);
        while (1<<i)<=n{
            if n&(1<<i) != 0 {
                println!("if => {}/{}",i,n);
                out = out.mt(&ar[i as usize]);
            }
            i+=1;
        }

        println!("out: {:#?}",out);
        let g = (out.a*6 + out.b*2 - 1 + 10000)%1000;
        
        
        // let g = out.get();
        if g<1{
            println!("Case #{}: 000",_t+1);
        }else if g<10{
            println!("Case #{}: 00{}",_t+1,g);
        }else if g<100{
            println!("Case #{}: 0{}",_t+1,g);
            
        }else{
            println!("Case #{}: {}",_t+1,g);
        }
        // println!("out:{:#?},get:{}",out,out.get());
        
    }
}