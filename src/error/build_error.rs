use crate::error::BuildErrorCast;
use std::fmt;

pub struct BuildError {
    display: String,
    debug: String,   
}

impl BuildError {
    pub (crate) fn from_error(error: impl fmt::Display + fmt::Debug) -> BuildError {
        let display = format!("{}", error);
        let debug = format!("{:?}", error);
        return BuildError {
            display,
            debug,
        }
    }

    pub fn display_str(&self) -> &str {
        return &self.display;
    }

    pub fn debug_str(&self) -> &str {
        return &self.debug;
    }
}

impl <T>From<T> for BuildError where T: BuildErrorCast {
    fn from(value: T) -> BuildError {
        return value.into_build_error();
    }
}