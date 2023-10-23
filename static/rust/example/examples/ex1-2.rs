fn main() {

//-------------------- mutable 변수 예제 --------------------//
    let mut aa: i32 = 1;
    let bb = 2;

    // mutable로 선언된 변수는 새로운 값 할당 가능
    aa = 3;

    let cc = aa + bb;

    println!("{} + {} = {}", aa, bb, cc);
}