fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sum1 = 0;
    let mut sum2 = 0;

    // 1부터 10까지 출력
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }
    println!();

    // 10부터 1까지 출력
    for i in (0..arr.len()).rev() {
        println!("arr[{}] = {}", i, arr[i]);
    }
    println!();

    // 1부터 10까지 더하기 - 인덱스를 이용하는 방법
    for i in 0..arr.len() {
        sum1 += arr[i];
    }
    println!("인덱스를 이용하는 방법");
    println!("1부터 10까지 합은 {sum1} 입니다");
    println!();

    // 1부터 10까지 더하기 - 이터레이터를 이용하는 방법
    for value in arr {
        sum2 += value;
    }
    println!("이터레이터를 이용하는 방법");
    println!("1부터 10까지 합은 {sum2} 입니다");
    println!();

    // 1부터 10까지 소수 찾기
    for num1 in 2..=10 {
        let mut is_prime = true;

        for num2 in 2..num1 {
            if num1 % num2 == 0 {
                is_prime = false;
                // break;
            }
        }

        if is_prime {
            println!("{num1}");
        }
    }
}