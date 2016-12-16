use libc::{intptr_t, uintptr_t};

pub use libc::{c_char, c_double, c_int, c_long, c_void, size_t};

#[cfg(unix)]
pub use std::os::unix::io::RawFd;

pub use typed_data::{RbDataType, RbDataTypeFunction};
pub use value::{Value, ValueType};

pub type Id = uintptr_t;
pub type InternalValue = uintptr_t;
pub type SignedValue = intptr_t;

pub type Argc = c_int;
pub type CallbackPtr = *const c_void;
pub type CallbackMutPtr = *mut c_void;

#[repr(C)]
pub struct RBasic {
    pub flags: InternalValue,
    pub klass: InternalValue,
}

#[link_name = "st_table"]
pub enum StTable {}

pub type StData = *const c_void;
pub type StMutData = *mut c_void;
