use std::marker::PhantomData;

#[derive(Debug)]
pub struct ArgumentId<T> {
    id: &'static str,
    _type: PhantomData<T>,
}

impl <T>ArgumentId<T> {
    pub const fn new(id: &'static str) -> ArgumentId<T> {
        return ArgumentId {
            id,
            _type: PhantomData,
        }
    }

    pub const fn get_id(&self) -> &'static str {
        return self.id;
    }
}