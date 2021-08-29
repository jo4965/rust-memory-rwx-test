//use nix::sys::mman::mprotect;
use libc;
use std::ffi::c_void;
use errno::{Errno, errno, set_errno};

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn hook() {


    let mut arr: [u8; 4] =[0xEB, 2, 3, 4];
    let arr_ptr = &arr as *const u8;

    let add_pointer = add as *mut u8;

    println!("add func address : {:x}", add_pointer as i64);
    let sub_pointer = sub as *mut u32;
    println!("arr address : {:x}", arr_ptr as usize);

    let off = ((add_pointer as i32) - (arr_ptr as i32 + 2)) as i64;
    println!("all func address : {:x}", ((add_pointer as i32) - (arr_ptr as i32)) );
    println!("all func address : {:x}", off);

//    arr[1] = off as u8;
//    println!("바로 윗줄의 메모리 주소를 입력해주세요 : ");

    unsafe {
        let add_ptr = std::mem::transmute::<*const u8, fn(i32, i32) -> i32>(arr_ptr);
        add_ptr(1, 5);
//        let pagesize = libc::sysconf(libc::_SC_PAGESIZE);
//        let pagestart = sub_pointer as i64 & -pagesize;
//        let end = sub_pointer as i64 + 1024;
//
//
//        let ret = libc::mprotect(pagestart as *mut _, 1024, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC);
//        println!("mprotect result : {}", ret);
//
//        let e = errno();
//
//        let code = e.0;
//
//        println!("Error {}: {}", code, e);
//
//        sub_pointer.write(0x33);
//        println!("sub pointer : {:x}", *sub_pointer.offset(0) );
//        sub_pointer.add(1).write(off);
//        addr_ptr.add(1).write(11); // 1 바이트 뒤로 움직여서 11로 덮어쓰기
//        addr_ptr.add(2).write(12);
//        addr_ptr.add(3).write(13);
    }

//    let a = add(5,2);
//    println!("add result : {}", a);
//    let b = sub(5,2);
//    println!("add result : {}", b);

}