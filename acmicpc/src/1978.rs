
fn readint()->Vec<i32>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}

fn main(){
    let _n = readint()[0];
    let ar = readint();

    let mut plist = [0;300];
    plist[0] = 2;
    let mut top:usize = 1;
    for i in 3..1000{
        let mut flag = true;
        for j in 0..top{
            if i%plist[j as usize] == 0{
                flag = false;
                break;
            }
        }
        if flag{
            plist[top] = i;
            top += 1;
        }
    }
    // println!("{:?}",plist);

    // for i in &ar{
    //     // println!("{}",plist.contains(&i))
    // }
    let d:i32 = ar.iter().map(|i| plist.contains(&i) as i32 ).sum();
    println!("{}",d);
}