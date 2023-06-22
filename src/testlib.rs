use std::fmt;

#[macro_export]
macro_rules! assert_approx_eq {
        ($left:expr, $right:expr $(,)?) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !((*left_val - *right_val).abs() < crate::EPSILON) {
                        approx_equals_fail(left_val, right_val, None);
                    }
                }
            }
        };
        ($left:expr, $right:expr, $($arg:tt)+) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        // The reborrows below are intentional. Without them, the stack slot for the
                        // borrow is initialized even before the values are compared, leading to a
                        // noticeable slow down.
                        approx_equals_fail(left_val, right_val, Option::Some(format_args!($($arg)+)));
                    }
                }
            }
        };
    }

#[track_caller]
pub fn approx_equals_fail(
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    args: Option<fmt::Arguments<'_>>,
) {
    let op = "!=";
    match args {
        Some(args) => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`: {}"#,
            op, left, right, args
        ),
        None => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`"#,
            op, left, right,
        ),
    }
}
