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
fn main() {
    check_case();
    uncheck_case();
}
