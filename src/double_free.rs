use std::vec::Vec;
pub fn check_case() {
    let mut a = vec![1,2];
    let ptr = a.as_mut_ptr();
    unsafe{
        let mut _v = Vec::from_raw_parts(ptr, 2, 2);
    }
}