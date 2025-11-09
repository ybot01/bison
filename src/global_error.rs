use alloc::boxed::Box;
use core::fmt::{self, Debug, Display};
use core::error::Error;

#[derive(Debug)]
pub enum GlobalError{
    Custom(&'static str),
    FromOtherError(Box<dyn Error + Send + 'static>)
}

impl<T: Error + Send + 'static> From<T> for GlobalError{
    fn from(value: T) -> Self {
        Self::FromOtherError(Box::new(value))
    }
}

pub type GlobalResult<T> = Result<T, GlobalError>;

impl Display for GlobalError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            GlobalError::Custom(value) => write!(f, "{}", value),
            GlobalError::FromOtherError(value) => write!(f, "{:?}", value)
        }
    }
}

#[allow(non_snake_case)]
impl GlobalError{
    pub const RETURNED_NONE: Self = Self::Custom("returned none");
}