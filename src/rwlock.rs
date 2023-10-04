use std::cell::UnsafeCell;

use serde::{Deserialize, Deserializer, Serialize};
use tokio::sync::Semaphore;

pub struct InternalRwLock<T: ?Sized> {
    _mr: u32,

    _s: Semaphore,

    c: UnsafeCell<T>,
}
pub struct RwLock<T>(pub tokio::sync::RwLock<T>);
impl<T> RwLock<T> {
    pub fn new(t: T) -> RwLock<T> {
        Self(tokio::sync::RwLock::new(t))
    }
    unsafe fn inner_unsafe(&self) -> *const InternalRwLock<T> {
        &self.0 as *const tokio::sync::RwLock<T> as *const InternalRwLock<T>
    }
}
impl<T> Default for RwLock<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
impl<T> Serialize for RwLock<T>
where
    T: Serialize,
    T: Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let unsafelock = unsafe { &*(self.inner_unsafe()) };
        let inner = unsafe { &*unsafelock.c.get().clone() };
        serializer.serialize_newtype_struct("RwLock", inner)
    }
}

impl<'de, T> Deserialize<'de> for RwLock<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<RwLock<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let a = Deserialize::deserialize(deserializer)?;

        Ok(RwLock(tokio::sync::RwLock::new(a)))
    }
}
