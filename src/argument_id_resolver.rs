use crate::argument_id::ArgumentId;
use std::any::{type_name, Any};

pub trait ArgumentIdResolver: Sized {
    const ARGUMENT_ID: ArgumentId<Self> = ArgumentId::new(type_name::<Self>());
}

impl <T>ArgumentIdResolver for T where T: Any {

}