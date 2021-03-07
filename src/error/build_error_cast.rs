use std::fmt::{Debug, Display};

use crate::error::BuildError;

pub trait BuildErrorCast {
    fn into_build_error(self) -> BuildError;
}

impl <T>BuildErrorCast for T 
    where 
        T: Display + Debug 
        {
            fn into_build_error(self) -> BuildError {
                return BuildError::from_error(&self);
            }
        }