

use std::io::{self};

// stdin으로 메모리 주소를 받아서 그 주소의 값을 변경합니다.
pub fn var_test() {
    // u8(byte) 4개 짜리 배열을 선언합니다.
    // 이 array를 메모리 접근을 통해 바꿀겁니다.
    let array: [u8; 4] = [1, 2, 3, 4];

    // 이게 primitive pointer입니다.
    // *const u8 또는 *mut u8 (u8 변수를 가리키는 포인터)
    let pointer_array = &array as *const u8; // C언어로치면 -> byte * pointer_array = array

    println!("데이터 내용 : {:?}", array);
    println!("데이터 메모리 주소 : {:x}", pointer_array as usize);

    // 위에 나온 메모리 주소를 받아서 i64 변수로 만들겁니다.
    println!("바로 윗줄의 메모리 주소를 입력해주세요 : ");

    // scanf 입니다
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("{}", buffer);

    // 받은 메모리 주소를 (16진수 string) -> (i64 자료형)으로 컨버팅 하는 부분입니다.
    let addr = i64::from_str_radix(buffer.trim_end(), 16).unwrap();

    println!("입력 받은 메모리 주소 : {:x}", addr);


    let addr_ptr = addr as *mut u8; // *mut u8은 *const u8와 달리 접근해서 값 수정이 가능합니다.

    unsafe {
        addr_ptr.write(10);
//        addr_ptr.add(1).write(11); // 1 바이트 뒤로 움직여서 11로 덮어쓰기
//        addr_ptr.add(2).write(12);
//        addr_ptr.add(3).write(13);
    }

    println!("변경된 데이터 내용 : {:?}", array);
}
