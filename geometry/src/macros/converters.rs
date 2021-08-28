#[macro_export]
macro_rules! str_to_f32 (
    ($str:expr) => ({
        $str.parse::<i32>().unwrap()
    })
);