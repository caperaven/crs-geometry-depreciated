#[macro_export]
macro_rules! str_to_f32 (
    ($str:expr) => ({
        $str.trim().parse::<f32>().unwrap()
    })
);