#[macro_export]
macro_rules! color_code {
    ($code: expr, $a: expr) => (format!("\x1b[{}m{}\x1b[0m", $code, $a))
}
#[macro_export]
macro_rules! black {
    ($a: expr) => (color_code!(30, $a))
}
#[macro_export]
macro_rules! red {
    ($a: expr) => (color_code!(31, $a))
}
#[macro_export]
macro_rules! green {
    ($a: expr) => (color_code!(32, $a))
}
#[macro_export]
macro_rules! yellow {
    ($a: expr) => (color_code!(33, $a))
}
#[macro_export]
macro_rules! blue {
    ($a: expr) => (color_code!(34, $a))
}
#[macro_export]
macro_rules! magenta {
    ($a: expr) => (color_code!(35, $a))
}
#[macro_export]
macro_rules! cyan {
    ($a: expr) => (color_code!(36, $a))
}
#[macro_export]
macro_rules! white {
    ($a: expr) => (color_code!(37, $a))
}
