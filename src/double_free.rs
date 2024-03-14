//! Double Free 的测试样例

use std::vec::Vec;

/// 可被检测到：Safedrop、Static-checker、MIRAI
pub fn check_case() {
    let mut a = vec![1, 2];
    let ptr = a.as_mut_ptr();
    unsafe {
        let mut _v = Vec::from_raw_parts(ptr, 2, 2);
    }
}

/// 不可被检测到：SafeDrop
/// 可被检测到：Static-checker、MIRAI
pub fn closure_case() {
    let mut a = vec![1, 2];
    let ptr = a.as_mut_ptr();
    let f = || unsafe {
        let mut _v = Vec::from_raw_parts(ptr, 2, 2);
    };
    f();
}

/// 函数指针样例
pub fn function_pointer_case() {
    let mut a = vec![1, 2];
    let ptr = a.as_mut_ptr();
    let f = || unsafe {
        let mut _v = Vec::from_raw_parts(ptr, 2, 2);
    };
    let f_pointer = Box::new(f);
    f_pointer();
}

/// 高阶函数样例
/// 可被检测到：Static-checker、MIRAI
pub fn high_order_function_case() {
    let mut a = vec![1, 2];
    let ptr = a.as_mut_ptr();
    let f = || unsafe {
        let mut _v = Vec::from_raw_parts(ptr, 2, 2);
    };
    high_order_function_caller(f);
}

fn high_order_function_caller<F: FnOnce() -> ()>(f: F) {
    f();
}

// use libc::{c_void, free};

// /// 不可被检测到：FFI-Checker
// /// 可被检测到：SafeDrop
// pub fn ffi_check_case() {
//     let mut n = Box::new(1);
//     unsafe {
//         free(&mut *n as *const _ as *mut c_void);
//     }
//     // *n = 2;
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

struct DummyType;
trait DymmyTrait {
    fn func(&self, _: *mut i32) {}
}
impl DymmyTrait for DummyType {
    fn func(&self, ptr: *mut i32) {
        unsafe {
            let mut _v = Vec::from_raw_parts(ptr, 2, 2);
        }
    }
}
/// 动态类型展示样例
pub fn dynamic_trait_show_case() {
    let mut a = vec![1, 2];
    let ptr = a.as_mut_ptr();
    let dummy_type = DummyType;
    let dyn_dymmy_trait: Box<dyn DymmyTrait> = Box::new(dummy_type);
    dyn_dymmy_trait.func(ptr);
}
