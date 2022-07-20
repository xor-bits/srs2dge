use crate::prelude::{GuiCalc, GuiCalcOffset, GuiCalcSize};

use self::{anim::Animated, lerp::Lerp};
use std::{
    borrow::Cow,
    fmt::Debug,
    sync::{Arc, RwLock},
};

//

pub mod anim;
pub mod lerp;
pub mod prelude;

//

#[derive(Debug, Clone)]
pub enum GuiValue<T> {
    Owned(T),
    Shared(Arc<RwLock<T>>),
    Animated(Animated<T>),
}

//

impl<T> GuiValue<T> {
    pub fn new(val: T) -> Self {
        Self::Owned(val)
    }

    pub fn new_shared(val: Arc<RwLock<T>>) -> Self {
        Self::Shared(val)
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> GuiValue<U>
    where
        T: Lerp + Clone,
    {
        match self {
            GuiValue::Owned(v) => GuiValue::Owned(f(v)),
            GuiValue::Shared(v) => GuiValue::Owned(f(v.read().unwrap().clone())),
            GuiValue::Animated(v) => GuiValue::Owned(f(v.get())),
        }
    }

    pub fn get(&self) -> Cow<T>
    where
        T: Lerp + Clone,
    {
        match self {
            GuiValue::Owned(owned) => Cow::Borrowed(owned),
            GuiValue::Shared(lock) => Cow::Owned(lock.read().unwrap().clone()),
            GuiValue::Animated(anim) => Cow::Owned(anim.get()),
        }
    }
}

impl<T: Default> Default for GuiValue<T> {
    fn default() -> Self {
        Self::Owned(T::default())
    }
}

impl<T> From<T> for GuiValue<T> {
    fn from(val: T) -> Self {
        GuiValue::Owned(val)
    }
}

impl From<Arc<dyn GuiCalc>> for GuiValue<GuiCalcSize> {
    fn from(val: Arc<dyn GuiCalc>) -> Self {
        GuiValue::Owned(val.into())
    }
}

impl From<Arc<dyn GuiCalc>> for GuiValue<GuiCalcOffset> {
    fn from(val: Arc<dyn GuiCalc>) -> Self {
        GuiValue::Owned(val.into())
    }
}

impl<T> From<T> for GuiValue<GuiCalcSize>
where
    T: GuiCalc + 'static,
{
    fn from(val: T) -> Self {
        GuiValue::Owned(val.into())
    }
}

impl<T> From<T> for GuiValue<GuiCalcOffset>
where
    T: GuiCalc + 'static,
{
    fn from(val: T) -> Self {
        GuiValue::Owned(val.into())
    }
}

impl<T> From<Arc<RwLock<T>>> for GuiValue<T> {
    fn from(val: Arc<RwLock<T>>) -> Self {
        GuiValue::Shared(val)
    }
}

impl<T> From<Animated<T>> for GuiValue<T> {
    fn from(val: Animated<T>) -> Self {
        GuiValue::Animated(val)
    }
}
