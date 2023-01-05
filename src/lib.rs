#![doc = include_str!("../README.md")]

use std::marker::PhantomData;

#[doc(hidden)]
pub struct Wrap<T>(PhantomData<T>);

impl<T> Wrap<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

#[doc(hidden)]
pub trait IsOption {
    fn is_option(&self) -> bool;
}

impl<T> IsOption for &Wrap<Option<T>> {
    fn is_option(&self) -> bool {
        true
    }
}

impl<T> IsOption for Wrap<T> {
    fn is_option(&self) -> bool {
        false
    }
}

/// Return `true` if the specified type is an option.
#[macro_export]
macro_rules! is_option {
    ($type:ty) => {{
        use $crate::{IsOption, Wrap};
        (&&Wrap::<$type>::new()).is_option()
    }};
}

#[cfg(test)]
mod test {
    use super::is_option;

    #[test]
    fn is_option() {
        assert!(!is_option!(usize));
        assert!(!is_option!(String));
        assert!(!is_option!(&str));
        assert!(is_option!(Option<usize>));
        assert!(is_option!(Option<&str>));
    }
}
