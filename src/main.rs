use std::vec::Vec;
fn check_case() {
    let mut idx = 0;
    let mut v = vec![1, 2, 3, 4, 5];
    while idx < 5 {
        idx = idx + 1;
    }
    v[idx] = idx;
}
fn uncheck_case() {
    let mut idx = 0;
    let mut v = vec![1, 2, 3, 4, 5];
    let mut f = || {
        while idx < 5 {
            idx = idx + 1;
        }
    };
    f();
    v[idx] = idx;
}
fn safedrop_case(){
    let mut a = vec![1,2];
    let ptr = a.as_mut_ptr();
    unsafe{
        let mut _v = Vec::from_raw_parts(ptr,2,2);
    }
}
fn main() {
    // check_case();
    // uncheck_case();
    safedrop_case();
}
