#![macro_use]
#[macro_export]
macro_rules! BIT {
    ($x:expr) => {
        1 << $x
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! MAGNUS_ASSERT {
    ($x:expr, $args:expr) => {
        if !x {
            error!("Assertion Failed: {}", $args);
            assert!(false);
        }
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! MAGNUS_CORE_ASSERT {
    ($x:expr, $args:expr) => {
        if !x {
            error!("Assertion Failed: {}", $args);
            assert!(false);
        }
    }
}
