use std::marker::PhantomData;
use std::fmt;

#[derive(Debug)]
pub struct ServiceId<T> {
    id: &'static str,
    _type: PhantomData<T>,
}

impl <T>ServiceId<T> {
    pub const fn new(id: &'static str) -> ServiceId<T> {
        return ServiceId {
            id,
            _type: PhantomData,
        }
    }

    pub const fn get_id(&self) -> &'static str {
        return self.id;
    }
}

impl <T> fmt::Display for ServiceId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}