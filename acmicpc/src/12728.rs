use std::io::*;

// #[derive(Debug)]
struct R{
    a:i64,
    b:i64,
}

impl R{
    fn new(a:i64, b:i64)->R{
        R{a,b}
    }
    fn mt(&self, m:&R)->R{
        R{a:(self.a*m.a + self.b*m.b*5)%10000, b:(self.a*m.b + self.b*m.a)%10000}
    }
    fn get(&self)->i64{
        ((2.23606797749979*(self.b as f64) + (self.a as f64)) as i64)%1000
    }
}
impl Copy for R {}
impl Clone for R {
    fn clone(&self) -> Self {
        R {a:self.a, b:self.b}
    }
}

fn readint()->Vec<i64>{
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}


fn main(){
    let t:i64 = readint()[0];
    let mut ar = vec![R::new(3,1);32];
    for i in 1..31{
        ar[i] = ar[i-1].mt(&ar[i-1]);
        // println!("{}답:{:#?}",i,ar[i]);
    }

    for _t in 0..t{
        let n:i64 = readint()[0];
        let mut i=0;
        let mut out = R{a:1,b:0};
        while (1<<i)<=n{
            if n&(1<<i) != 0 {
                println!("if => {}/{}",i,n);
                out = out.mt(&ar[i as usize]);
            }
            i+=1;
        }
        
        
        let g = out.get();
        println!("Case #{}: {:<03}",_t+1,g);
        // if g<1{
        //     println!("Case #{}: 000",_t+1);
        // }else if g<10{
        //     println!("Case #{}: 00{}",_t+1,g);
        // }else if g<100{
        //     println!("Case #{}: 0{}",_t+1,g);
            
        // }else{
            
        // }
        // println!("out:{:#?},get:{}",out,out.get());
        
    }
}