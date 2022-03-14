extern crate rand;
use std::io;
use rand::Rng;
fn main(){
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("숫자 추측!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("읽기 실패");
    println!("너의 추측{}",guess)

}