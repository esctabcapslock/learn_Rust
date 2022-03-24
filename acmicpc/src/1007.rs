use std::io::*;

fn readint()->Vec<i32>{
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}

fn conv(n:usize, index:usize, cnt:usize, ar:&Vec<Vec<i32>>, sumx:i32, sumy:i32,k:i32)->f32{
    // println!("n:{}, index:{},cnt:{}, sum:({},{})",n,index,cnt, sumx,sumy);
    if index==n-1{
        // println!("if");
        let f = if cnt==n/2{-1}else{1}; //마지막이 1로 더하는 경우
        let sumx = (sumx + f*ar[index][0]) as f32;
        let sumy = (sumy + f*ar[index][1]) as f32;
        // println!("if =? {},{}, {}",k,cnt,(sumx*sumx+sumy*sumy).sqrt());

        return (sumx*sumx+sumy*sumy).sqrt();
    }else{
        // println!("el");
        let k =k;
        let mut m = 300000.1;
        // i=5, cnt==5 불가능
        // i=6, cnt=1
        if index-cnt < n/2{
        m = conv(n, index+1, cnt, ar, sumx-ar[index][0], sumy-ar[index][1],k*2).min(m); //다음 숫자는 뺀 숫자다.
        }
        if cnt<n/2{  //더한것도 가능하다
            m = conv(n, index+1, cnt+1, ar, sumx+ar[index][0], sumy+ar[index][1],k*2+1).min(m);
        }
        return m;
    }
    
}


fn main(){
    let t:i32 = readint()[0];
    for _ in 0..t{
        let n:usize = readint()[0] as usize;
        let mut ar =vec![vec![0;2]; n];
        for i in 0..n{
            ar[i] = readint();
        }

        // 노가다
        // println!("{:?}",ar);
        let min = conv(n,0,0,&ar,0,0,0);
        println!("{}",min);
        // println!("min:: {:?}",min);
    }
}