pub mod timeframe;

pub type BoxedError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, BoxedError>;
