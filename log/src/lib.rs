#[macro_export]
macro_rules! brs_dbg {
    ($content:ident) => {
        #[cfg(feature = "debug")]
        dbg!($content)
    };
}