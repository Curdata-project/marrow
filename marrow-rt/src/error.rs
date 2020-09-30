use core::cell::{BorrowError, BorrowMutError};
use core::fmt;

#[derive(Debug)]
pub enum Error {
    NoRuningModule,
    NoMethod,
    ModuleTypeError,
    WasmiError(wasmi::Error),
    WasmiTrap(wasmi::Trap),
    BorrowMutError(BorrowMutError),
    BorrowError(BorrowError),
}

impl From<wasmi::Error> for Error {
    fn from(e: wasmi::Error) -> Error {
        Error::WasmiError(e)
    }
}

impl From<wasmi::Trap> for Error {
    fn from(e: wasmi::Trap) -> Error {
        Error::WasmiTrap(e)
    }
}

impl From<BorrowMutError> for Error {
    fn from(e: BorrowMutError) -> Error {
        Error::BorrowMutError(e)
    }
}

impl From<BorrowError> for Error {
    fn from(e: BorrowError) -> Error {
        Error::BorrowError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl wasmi::HostError for Error {}
