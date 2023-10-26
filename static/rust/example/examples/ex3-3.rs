fn main() {
//-------------------- 반환값이 있는 함수 예제 --------------------//
    let a = 0;
    let ret = plus_one(a);

    println!("returned value: {}", ret);
}

fn plus_one(value: i32) -> i32 {
    println!("{} + 1 = {}", value, value + 1);

    // return value + 1;    // return 함수를 사용해 변수를 반환할 수도 있다
    value + 1
}