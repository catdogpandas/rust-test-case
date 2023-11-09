//! Double Free 的测试样例
//! 

use std::vec::Vec;

/// 可被检测到：Safedrop
pub fn check_case() {
    let mut a = vec![1,2];
    let ptr = a.as_mut_ptr();
    unsafe{
        let mut _v = Vec::from_raw_parts(ptr, 2, 2);
    }
}

/// 不可被检测到：SafeDrop
pub fn uncheck_case(){
    let mut a = vec![1,2];
    let ptr = a.as_mut_ptr();
    let f = ||{
        unsafe{
            let mut _v = Vec::from_raw_parts(ptr, 2, 2);
        }
    };
    f();
}