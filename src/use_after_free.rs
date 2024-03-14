//! Use After Free 测试样例


/// 不可被检测到：FFI-Checker
/// 可被检测到：SafeDrop、Mir-Checker
pub fn check_case() {
    let v = {
        let mut s = vec![1, 2, 3, 4, 5];
        let ptr = s.as_mut_ptr();
        unsafe { Vec::from_raw_parts(ptr, s.len(), s.len()) }
    };
    println!("v: {:?}", v);
}

/// 不可被检测到：SafeDrop、FFI-Checker
pub fn uncheck_case() {
    let v = {
        let mut s = vec![1, 2, 3, 4, 5];
        let mut f = || {
            let ptr = s.as_mut_ptr();
            unsafe { Vec::from_raw_parts(ptr, s.len(), s.len()) }
        };
        f()
    };
    println!("v: {:?}", v);
}

// use libc::{c_void, free};
// /// 可被检测到：FFI-Checker
// pub fn ffi_check_case() {
//     let mut n = Box::new(1);
//     unsafe {
//         free(&mut *n as *const _ as *mut c_void);
//     }
//     *n = 2;
// }

// /// 不可被检测到：FFI-Checker
// pub fn ffi_uncheck_case() {
//     let mut n = Box::new(1);
//     let f_do = || unsafe {
//         free(&mut *n as *const _ as *mut c_void);
//     };
//     let mut f_caller: Box<dyn FnMut() -> ()> = Box::new(f_do);
//     f_caller();
//     // *n = 2;
// }
