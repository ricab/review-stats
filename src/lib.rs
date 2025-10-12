pub mod timeframe;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, BoxedError>;
