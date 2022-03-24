# 백준 문제풀기

## 1001~
- 입력받는 법 찾기 ![이렇게](https://comb.tistory.com/9)

## 인상적인 코드들

### 정수 입력을 간편하게
```rs
fn readint()->Vec<i64>{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.split_whitespace().map(|s| s.trim().parse().expect("이상한 수임")).collect::<Vec<_>>()
}
```

### 늘 햇갈리는 나머지 연산

```rs
fn main() {
    for i in -10..10{
        print!("{}, ",i%1000);
    }
}
```
이거 찍으면 `-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, ` 출력됨...

### 출력할때 형식

```rs
println!("{0:<03}", (2 * ans + 1000 - 1) % 1000)
```
이렇게 하면 python의 f-string 약간 비슷함
    
    >>> f'{3.3425:.3f}'
    '3.342'
였음

```rs
k=r"123
23"
```
이런식 하면 여러줄 넣을수 있음
```rs
k=r#"123 \
23"#
```
이러면 아무런 제약없이 쓸 수 있음

### 디버그
```rs
#[derive(Debug)]
```
구조체 위에 적고, 
```rs
println!("{:#?}",mystruct_instance);
```
와 같이 출력한다.

### 크기비교
```rs
    match a[0].cmp(&a[1]){
        Ordering::Less => println!("<"),
        Ordering::Greater => println!(">"),
        Ordering::Equal => println!("=="),
    }
```

이 풀이 기억해두자 (34831332)
```rs
fn main() {
    let mut score = String::new();
    std::io::stdin().read_line(&mut score).unwrap();
    println!("{}", match score.trim().parse().unwrap() {90..=100 => "A", 80..=89 => "B", 70..=79 => "C", 60..=69 => "D", _ => "F"});
}
```

### 풀까
- [이거](https://www.acmicpc.net/problem/3955)