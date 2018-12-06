#![macro_use]
#[macro_export]
macro_rules! BIT {
    ($x:expr) => {
        1 << $x
    }
}
