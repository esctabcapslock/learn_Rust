extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("숫자 추측!");


    loop{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("읽기 실패");
    
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    println!("너의 추측{}",guess);
    match guess.cmp(&secret_number){
        Ordering::Less => println!("작음"),
        Ordering::Greater => println!("큼"),
        Ordering::Equal => {
            println!("동일");
            break;
        },
    }
}
}