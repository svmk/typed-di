use crate::error::BuildError;

pub trait BuildErrorCast {
    fn into_build_error(self) -> BuildError;
}