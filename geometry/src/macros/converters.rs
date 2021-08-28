#[macro_export]
macro_rules! str_to_i32 (
    ($str:expr) => ({
        $str.trim().parse::<i32>().unwrap()
    })
);