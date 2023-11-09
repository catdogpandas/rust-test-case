//! Division by Zero 测试样例

#[allow(unused_variables)]
#[allow(unconditional_panic)]
pub fn check_case(){
    let n = 0;
    let a = 100;

    verify!(n == 0);
    let b = a / n; // Error: division by zero!

    if n != 0 {
        verify!(n != 0);
        let c = a / n; // OK
    }
}