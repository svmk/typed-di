use std::error::Error;

pub type BuildError = Box<dyn Error + Send + Sync>;