fn main() {

//-------------------- shadowing 기능 예제 --------------------//
    let var: &str= "hello rust"; // 문자형 변수 선언
    println!("first assignment: {}", var);

    // shadowing 기능을 사용하면 동일한 변수 이름으로 재선언 가능
    let var: f64 = 1.56; // 실수형으로 재선언
    println!("second assignment: {}", var);
}