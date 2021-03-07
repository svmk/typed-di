use crate::argument::argument_id::ArgumentId;
use std::any::{type_name, Any};

pub trait ArgumentIdResolver: Sized {
    const ARGUMENT_ID: ArgumentId<Self> = ArgumentId::from_str(type_name::<Self>());
}

impl <T>ArgumentIdResolver for T where T: Any {

}