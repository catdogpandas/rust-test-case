pub mod buffer_overflow;
pub mod division_by_zero;
pub mod dop;
pub mod double_free;
pub mod examples;
pub mod integer_overflow;
pub mod mirchecker_panic;
pub mod out_of_bounds_access;
pub mod use_after_free;

fn main() {
    buffer_overflow::check_case();
    buffer_overflow::closure_case();
    buffer_overflow::function_pointer_case();
    buffer_overflow::high_order_function_case();

    double_free::check_case();
    double_free::closure_case();
    double_free::function_pointer_case();
    double_free::high_order_function_case();

    use_after_free::check_case();
    use_after_free::uncheck_case();

    out_of_bounds_access::check_case();
    out_of_bounds_access::uncheck_case();

    integer_overflow::check_case();
    integer_overflow::uncheck_case();

    division_by_zero::check_case();
    division_by_zero::uncheck_case();

    // show case
    buffer_overflow::show_case();
    double_free::dynamic_trait_show_case();
    dop::dop();
    mirchecker_panic::mirchecker_panic();
}
