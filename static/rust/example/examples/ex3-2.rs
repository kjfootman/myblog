fn main() {
//-------------------- 함수 인자 예제 --------------------//
    let a = 0;

    plus_one(a);
}

//-------------------- 함수 인자 예제 --------------------//
fn plus_one(value: i32) {
    println!("{} + 1 = {}", value, value + 1);
}