//! Division by Zero 测试样例

/// 可被检测到：
/// 不可被检测到：SafeDrop
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn check_case(){
    let mut n = 0;
    let a = 100;

    // 逃避默认检测
    if n == 0 {
        n = n * n;
    }

    let b = a / n; // Error: division by zero!
    println!("b: {}", b);
}