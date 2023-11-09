//! Buffer overflow 测试样例

#[repr(C)]
struct Hackvist {
    buffer: [u8; 16],
    point: *const fn(),
}

/// 可被检测到：
/// 不可被检测到：SafeDrop
pub(crate) fn check_case() {
    let input_string = String::from("testtesttesttestAAAAAAAA");
    let input_bytes: &[u8] = input_string.as_bytes();
    let mut hackvist = Hackvist {
        buffer: [0; 16],
        point: 0 as *const fn() -> (),
    };

    unsafe {
        std::ptr::copy(
            input_bytes.as_ptr(),
            hackvist.buffer.as_mut_ptr(),
            input_bytes.len(),
        )
    }

    println!(
        "hackvist.point after strcpy: x{:0x}",
        hackvist.point as usize
    );
}
