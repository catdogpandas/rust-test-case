//! Division by Zero 测试样例

/// 可被检测到：Mir-Checker
/// 不可被检测到：SafeDrop，FFI-Checker
#[allow(dead_code)]
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

/// 不可被检测到：Mir-Checker
#[allow(dead_code)]
pub fn uncheck_case(){
    let mut n = 0;
    let a = 100;

    // 逃避默认检测
    if n == 0 {
        n = n * n;
    }
    
    let f = ||{
        let b = a / n; // Error: division by zero!
        println!("b: {}", b);
    };
    f();

    // let b = a / n; // Error: division by zero!
    // println!("b: {}", b);
}