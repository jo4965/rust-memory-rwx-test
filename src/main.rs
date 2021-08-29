
mod var_pointer_test;
mod func_pointer_test;
mod func_hook;


fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

// stdin으로 메모리 주소를 받아서 그 주소의 값을 변경합니다.
fn main() {
//    varPointerTest::var_test();
//    funcPointerTest::func_test();
    func_hook::hook();
//    let a = add(5,2);
//    println!("add result : {}", a);
//    let b = sub(5,2);
//    println!("add result : {}", b);
}
