use std::fmt;
use std::any::TypeId;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ServiceName {
    String(String),
    StaticString(&'static str),
    Uuid(u128),
    TypeId(TypeId),
}

impl fmt::Debug for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceName::String(service_name) => {
                write!(f, "ServiceName({})", service_name)
            },
            ServiceName::StaticString(service_name) => {
                write!(f, "ServiceName({})", service_name)
            },
            ServiceName::Uuid(uuid) => {
                let uuid = format!("{:032x}", uuid);
                write!(f, "ServiceName({}-{}-{}-{}-{})", &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32])
            },
            ServiceName::TypeId(type_id) => {
                write!(f, "ServiceName({:?})", type_id)
            },
        }
    }
}