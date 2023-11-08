//! 整数溢出的测试样例
//!

/// 整数溢出检出样例
pub fn check_case() {
    let x: u8 = 222;
    let mut y: u8 = 222;
    // 增加if else后，可逃避默认检测
    if y == 222 {
        y = 255;
    } else {
        y = 1;
    }

    // 如果不加溢出检查，下面的操作将导致 panic
    let sum = x + y;
    println!("Sum is: {}", sum);

    // 使用溢出检查的方法
    // let sum = x.checked_add(y);

    // match sum {
    //     Some(result) => {
    //         println!("Sum is: {}", result);
    //     }
    //     None => {
    //         println!("Overflow occurred");
    //     }
    // }
}
