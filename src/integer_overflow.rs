//! 整数溢出的测试样例
//!

/// 可被检测到：Mir-Checker
/// 不可被检测到：SafeDrop
pub fn check_case() {
    let x: u8 = 222;
    let mut y: u8 = 222;

    // 逃避默认检测
    if y == 222 {
        y = 255;
    }

    // 如果不加溢出检查，下面的操作将导致 panic
    let sum = x + y;
    println!("Sum is: {}", sum);

    // 使用溢出检查的方法
    // let sum = x.checked_add(y);
}

/// 不可被检测到：Mir-Checker
pub fn uncheck_case() {
    let x: u8 = 2;
    let mut y: u8 = 2;

    // 逃避默认检测
    let mut f = || {
        y = 255;
    };
    f();
    // 如果不加溢出检查，下面的操作将导致 panic
    let sum = x + y;
    println!("Sum is: {}", sum);

    // 使用溢出检查的方法
    // let sum = x.checked_add(y);
}
