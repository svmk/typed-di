use crate::argument::argument_name::ArgumentName;
use std::marker::PhantomData;
use std::any::{Any, type_name, TypeId};

#[derive(Debug)]
pub struct ArgumentId<T> {
    name: ArgumentName,
    _type: PhantomData<T>,
}

impl <T>ArgumentId<T> {
    pub const fn from_str(name: &'static str) -> ArgumentId<T> {
        return ArgumentId {
            name: ArgumentName::StaticString(name),
            _type: PhantomData,
        }
    }

    pub const fn from_string(name: String) -> ArgumentId<T> {
        return ArgumentId {
            name: ArgumentName::String(name),
            _type: PhantomData,
        }
    }

    pub const fn from_uuid(name: u128) -> ArgumentId<T> {
        return ArgumentId {
            name: ArgumentName::Uuid(name),
            _type: PhantomData,
        }
    }

    pub const fn from_type_id(name: TypeId) -> ArgumentId<T> {
        return ArgumentId {
            name: ArgumentName::TypeId(name),
            _type: PhantomData,
        }
    }

    pub const fn get_name(&self) -> &ArgumentName {
        return &self.name;
    }

    pub const fn typename() -> ArgumentId<T> where T: Any {
        return ArgumentId::from_str(type_name::<T>());
    }
}