use crate::error::Error;
use std::sync::{Arc};
use std::any::{Any, type_name};
use std::clone::Clone;

#[derive(Debug)]
pub struct ArgumentInstance {
    id: &'static str,
    argument: Arc<Box<dyn Any>>,
}

impl ArgumentInstance {
    pub fn new<T>(id: &'static str, argument: T) -> ArgumentInstance
        where
            T: 'static,
            T: Any,
            T: Send,
            T: Sync,
    {
        return ArgumentInstance {
            id,
            argument: Arc::new(Box::new(argument)),
        }
    }

    pub fn downcast_ref<T>(&self) -> Result<&T, Error> where T: Any, T: 'static {
        match self.argument.downcast_ref::<T>() {
            Some(argument) => {
                return Ok(argument);
            },
            None => {
                return Err(Error::argument_downcast(self.id.to_string(), type_name::<T>().to_string()))
            },
        }
    }
}

unsafe impl Sync for ArgumentInstance {}
unsafe impl Send for ArgumentInstance {}

impl Clone for ArgumentInstance {
    fn clone(&self) -> Self {
        return ArgumentInstance {
            id: self.id,
            argument: self.argument.clone(),
        }
    }
}