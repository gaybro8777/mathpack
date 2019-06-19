#![allow(unused_macros)]
#![deny(clippy::all, clippy::pedantic)]
pub mod calculus;
pub mod constants;
pub mod linalg;
pub mod statistics;
pub mod types;

#[macro_export]
macro_rules! create_equality_test {
    ($name: ident, $f: expr, $( $var:expr ),* => $eq: expr) => {
        #[test]
        fn $name() {
            assert_eq!($eq, $f($($var),*));
        }
    };
}

#[macro_export]
macro_rules! create_inequality_test {
    ($name: ident, $f: expr, $( $var:expr ),* => $eq: expr) => {
        #[test]
        fn $name() {
            assert_ne!($eq, $f($($var),*));
        }
    };
}

#[macro_export]
macro_rules! create_panic_test {
    ($name: ident, $f: expr, $( $var:expr ),*) => {
        #[test]
        #[should_panic]
        fn $name() {
            $f($($var),*);
        }
    };
}
