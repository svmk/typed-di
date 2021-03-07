use crate::argument::argument_name::ArgumentName;
use std::fmt;
#[derive(Debug)]
pub struct ArgumentDowncastError {
    id: ArgumentName,
    argument_type: String,
}

impl ArgumentDowncastError {
    pub fn new(
        id: ArgumentName,
        argument_type: String,
    ) -> ArgumentDowncastError {
        return ArgumentDowncastError {
            id,
            argument_type,
        }
    }
}

impl fmt::Display for ArgumentDowncastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to downcast argument `{:?}` to type `{}`", self.id, self.argument_type)
    }
}

impl std::error::Error for ArgumentDowncastError {}