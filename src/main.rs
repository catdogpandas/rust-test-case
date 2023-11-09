pub mod buffer_overflow;
pub mod mirchecker_panic;
pub mod dop;
pub mod double_free;
pub mod out_of_bounds_access;
pub mod integer_overflow;

fn main() {
    buffer_overflow::check_case();
    // mirchecker_panic::mirchecker_panic();
    // dop::dop();
    // double_free::check_case();
    // double_free::uncheck_case();
    // out_of_bounds_access::check_case();
    // out_of_bounds_access::uncheck_case();
    // integer_overflow::check_case();
}
