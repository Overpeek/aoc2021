pub mod a;
pub mod b;

#[cfg(not(feature = "test"))]
pub const DATA: &str = include_str!("data");
#[cfg(feature = "test")]
pub const DATA: &str = include_str!("test");
