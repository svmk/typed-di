use crate::error::Error;
use crate::argument::argument_name::ArgumentName;
use std::sync::{Arc};
use std::any::{Any, type_name};
use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct ArgumentInstance {
    name: ArgumentName,
    argument: Arc<Box<dyn Any>>,
}

impl ArgumentInstance {
    pub fn new<T>(name: ArgumentName, argument: T) -> ArgumentInstance
        where
            T: 'static,
            T: Any,
            T: Send,
            T: Sync,
    {
        return ArgumentInstance {
            name,
            argument: Arc::new(Box::new(argument)),
        }
    }

    pub fn downcast_ref<T>(&self) -> Result<&T, Error> where T: Any, T: 'static {
        match self.argument.downcast_ref::<T>() {
            Some(argument) => {
                return Ok(argument);
            },
            None => {
                return Err(Error::argument_downcast(self.name.clone(), type_name::<T>().to_string()))
            },
        }
    }
}

unsafe impl Sync for ArgumentInstance {}
unsafe impl Send for ArgumentInstance {}