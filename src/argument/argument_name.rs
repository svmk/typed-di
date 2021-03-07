use std::fmt;
use std::any::TypeId;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ArgumentName {
    String(String),
    StaticString(&'static str),
    Uuid(u128),
    TypeId(TypeId),
}

impl fmt::Debug for ArgumentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentName::String(argument_name) => {
                write!(f, "ArgumentName({})", argument_name)
            },
            ArgumentName::StaticString(argument_name) => {
                write!(f, "ArgumentName({})", argument_name)
            },
            ArgumentName::Uuid(uuid) => {
                let uuid = format!("{:032x}", uuid);
                write!(f, "ArgumentName({}-{}-{}-{}-{})", &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32])
            },
            ArgumentName::TypeId(type_id) => {
                write!(f, "ArgumentName({:?})", type_id)
            },
        }
    }
}