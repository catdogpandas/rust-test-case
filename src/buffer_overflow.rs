//! Buffer overflow 测试样例

/// 栈溢出样例
pub fn check_case() {
    let x = 2;
    let mut buf = Vec::new();
    buf.push(0);
    buf.push(1);
    buf[x] = 2;
}

/// 栈溢出闭包样例
pub fn closure_case() {
    let mut buf = Vec::new();
    buf.push(0);
    let f = || 2;
    let x = f();
    buf[x] = 2;
}

/// 栈溢出函数指针样例
pub fn function_pointer_case() {
    let mut buf = Vec::new();
    buf.push(0);
    let f = || 2;
    let f_pointer = Box::new(f);
    let x = f_pointer();
    buf[x] = 2;
}

/// 栈溢出高阶函数样例
pub fn high_order_function_case() {
    let mut buf = Vec::new();
    buf.push(0);
    let f = || 2;
    let x = high_order_function_caller(f);
    buf[x] = 2;
}

fn high_order_function_caller<F: FnOnce() -> usize>(f: F) -> usize {
    f()
}

/// 展示栈溢出漏洞利用
#[repr(C)]
struct Hackvist {
    buffer: [u8; 16],
    point: *const fn(),
}

/// 可被检测到：
/// 不可被检测到：SafeDrop、Mir-Checker
#[allow(dead_code)]
pub(crate) fn show_case() {
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
