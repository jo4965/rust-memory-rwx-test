use std::io::{self};

fn add(x: i32, y: i32) -> i32 {
    x + y
}


pub fn func_test() {

    let func_pointer = add as i64;

    println!("add 함수 메모리 주소 : {:x}", func_pointer as usize);
    println!("바로 윗줄의 메모리 주소를 입력해주세요 : ");

    // scanf 입니다
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("{}", buffer);

    // 받은 메모리 주소를 (16진수 string) -> (i64 자료형)으로 컨버팅 하는 부분입니다.
    let new_func_addr = i64::from_str_radix(buffer.trim_end(), 16).unwrap();

    println!("입력 받은 메모리 주소 : {:x}", new_func_addr);

    type Binop = fn(i32, i32) -> i32;
    unsafe {
        let add_ptr = std::mem::transmute::<i64, Binop>(new_func_addr);
        println!("add({}, {}) : {}", 1, 5, add_ptr(1, 5));
    }

}
