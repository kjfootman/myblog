fn main() {
    let a = 4;
    let b = 1;
    let c = 5;

    // 홀수-짝수 비교
    if (a % 2) == 0 {
        println!("a는 짝수 입니다.");
    } else {
        println!("a는 홀수 입니다.");
    }

    // a와 b에 저장된 값 동일한지 비교
    if a == b {
        println!("a와 b는 같은 값 입니다.");
    } else {
        println!("a와 b는 다른 값 입니다.");
    }

    // a, b, c 크기 비교
    if b < a {
        println!("b는 a보다 작은 수 입니다.");
    } else if b > c {
        println!("b는 c보다 큰 수 입니다.");
    } 
    // else if a <= b && b < 3 {
    //     println!("b는 a보다 크거나 같고 3보다 작습니다.")
    // } 
    else {
        println!("b는 a보다 크거나 같고 c보다 작거나 같습니다.");
    }

    // 조건에 따른 변수값 할당
    let d = if c % 2 == 0 {
        0.0
    } else {
        1.0
    };

    println!("변수 d에 저장된 값은 {d} 입니다.");
}