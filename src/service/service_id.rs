use crate::service::service_name::ServiceName;
use std::{any::TypeId, marker::PhantomData};
use std::any::{Any, type_name};

#[derive(Debug)]
pub struct ServiceId<T> {
    name: ServiceName,
    _type: PhantomData<T>,
}

impl <T>ServiceId<T> {
    pub const fn from_str(name: &'static str) -> ServiceId<T> {
        return ServiceId {
            name: ServiceName::StaticString(name),
            _type: PhantomData,
        }
    }

    pub const fn from_string(name: String) -> ServiceId<T> {
        return ServiceId {
            name: ServiceName::String(name),
            _type: PhantomData,
        }
    }

    pub const fn from_uuid(name: u128) -> ServiceId<T> {
        return ServiceId {
            name: ServiceName::Uuid(name),
            _type: PhantomData,
        }
    }

    pub const fn from_type_id(name: TypeId) -> ServiceId<T> {
        return ServiceId {
            name: ServiceName::TypeId(name),
            _type: PhantomData,
        }
    }

    pub const fn get_name(&self) -> &ServiceName {
        return &self.name;
    }

    pub const fn typename() -> ServiceId<T> where T: Any {
        let name = type_name::<T>();
        return ServiceId::from_str(name);
    }
}