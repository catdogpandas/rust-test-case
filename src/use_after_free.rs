//! Use After Free 测试样例

/// 可被检测到：SafeDrop
pub fn check_case() {
    let v = {
        let mut s = vec![1, 2, 3, 4, 5];
        let ptr = s.as_mut_ptr();
        unsafe { Vec::from_raw_parts(ptr, s.len(), s.len()) }
    };
    println!("v: {:?}", v);
}

/// 不可被检测到：SafeDrop
pub fn uncheck_case(){
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
