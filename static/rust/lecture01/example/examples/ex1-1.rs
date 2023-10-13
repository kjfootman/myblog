fn main() {
//----- immutable 변수 예제 -----//
    let a: i32 = 1;
    let b = 2;
    let c = a + b;

    // imutable로 선언된 변수에 새로운 값 할당을 시도할 경우 에러 발생
    // a = 3;
    println!("{} + {} = {}", a, b, c);

//----- immutable 변수 예제 -----//
    let mut _aa: i32 = 1;
    let bb = 2;

    // mutable로 선언된 변수에 새로운 값 할당 가능
    _aa = 3;
    let cc = _aa + bb;
    println!("{} + {} = {}", _aa, bb, cc);
}