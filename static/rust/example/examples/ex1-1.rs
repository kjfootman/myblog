fn main() {

//-------------------- immutable 변수 예제 --------------------//
    let a: i32 = 1;
    let b = 2;
    let c = a + b;

    // imutable로 선언된 변수에 새로운 값을 할당할 경우 에러 발생
    // a = 3;

    println!("{} + {} = {}", a, b, c);
}