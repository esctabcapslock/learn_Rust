// 안전하지 못한 코드를 써보자
// 2022-05-25
// https://rinthel.github.io/rust-lang-book-ko/ch19-01-unsafe-rust.html
// cd p_19_unsafe


// 안전하지 못한 Rust로 전환하기 위하여 unsafe 키워드를 활용한다.

// unsafe가 할 수 있는 일들.

// 로우 포인터 (raw pointer) 를 역참조하기
// 안전하지 않은 함수 혹은 메소드 호출하기
// 가변 정적 변수 (mutable static variable) 의 접근 혹은 수정하기
// 안전하지 않은 트레잇 구현하기

// 할 수 없는 일들

// 기존 참조자 검사 안하기

// 해야 할 일들

// unsafe 코드를 추상화하여 감싸기


fn raw_pointer() {
    println!("Hello, world!");

    let mut num = 5;
    
    //역참조 생성, as *mut 부분이 raw 포인터로 바꾸는 부분이다.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &mut num as *mut i32;

    let mut _r3_1 = &mut num;
    _r3_1 = &mut 23;


    // let 앞에 붙는 mut는 변경가능한 변수라는 뜻
    // 변수 앞에 &mut ('mut' 는 안됨)은 그 변수의 가변 참조자 생성

    const _R:u32 = 43;

    let r4 = &num;
    let mut r5 = &num;
    let mut r6 = &num; // 두 번 되는데??
    r5 = &4;
    r6 = &r5;
    // r4 = &3; // 불가능. r4는 mut 아님

    let r7 = *r6;

    // let r8 = *r1; // 이 옵션은 unsafe 안에서 실행해야 한다.
    let num = 55;
    unsafe{
        let r8 = *r1;
        // *r1 = 1; // mut가 아니라 변경이 불가능.
        *r2 = 2;

        println!("{}, {}, {}, {}, {}, {}, {}, {}",*r1,*r2,*r3,r4,r5,r6, r7,r8);
        //2, 2, 2, 2, 4, 4, 4, 5
    }




}

// 안전하지 않는 함수

fn unsafe_fn(){
    println!("[unsafe_fn]");

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

}

fn split(){
    println!("[split]");

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(mid <= len);
    
        // (&mut slice[..mid],
        //  &mut slice[mid..])
        // cannot borrow `*slice` as mutable more than once at a time
        // second mutable borrow occurs here

        // 그러나,
        // 슬라이스의 서로 다른 부분을 빌리는 것은 이 두 슬라이스가 서로 겹치지 않기 때문에 근본적으로 괜찮다.


        
        let ptr = slice.as_mut_ptr();
        use std::slice;
        unsafe {
            (slice::from_raw_parts_mut(ptr, mid),
             slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    //let r = &mut v;// 아랫것과 같은 정의인
    let r = &mut v;[..];//[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    use std::slice;

    let address = 0x012345usize;
    let r = address as *mut i32;

    // 임의의 메모리 위치로부터 슬라이스 생성하기
    // 안전하다는 보정은 없다.
    let slice = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };

}


fn foreign_function_interface(){
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


fn main(){
    use std::env;
    let args: Vec<String> = env::args().collect();
    let i = args[1].trim().parse().expect("Wanted a number");
    println!("index i: {}",i);
    // let i = 0;
    match i {
        0 => raw_pointer(),
        1 => unsafe_fn(),
        2 => split(),
        3 => foreign_function_interface(),
        _ => panic!("no number")
    }
}