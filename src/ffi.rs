//! Low level definitions for hpy ABI
pub use crate::context::*;
pub use libc::*;
pub type HPy_ssize_t = libc::intptr_t;
use std::ptr;

/// Same as in `hpy.h`, this is a `struct` so that equality (`PartialEq` in Rust) is
/// not implemented (would be confusing for application code authors)
#[derive(Clone, Debug)]
pub struct HPy(HPy_ssize_t);

// (comment from hpy's `meth.h`): make sure to use a bit which is unused by CPython
pub const _HPy_METH: c_int = 0x100000;
pub const HPy_METH_VARARGS: c_int = (0x0001 | _HPy_METH);
pub const HPy_METH_KEYWORDS: c_int = (0x0003 | _HPy_METH);
// (comment from hpy's `meth.h`):
// METH_NOARGS and METH_O must not be combined with the flags above.
pub const HPy_METH_NOARGS: c_int = (0x0004 | _HPy_METH);
pub const HPy_METH_O: c_int = (0x0008 | _HPy_METH);

// Temporay placeholder
pub struct PyObject;

pub type _HPy_CPyCFunction =
    unsafe extern "C" fn(slf: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
pub type HPyMeth =
    unsafe extern "C" fn(out_func: *mut *mut c_void, out_trampoline: *mut _HPy_CPyCFunction);

#[repr(C)]
pub struct HPyMethodDef {
    pub ml_name: *const c_char, // The name of the built-in function/method
    pub ml_meth: Option<HPyMeth>,
    pub ml_flags: c_int, // Combination of METH_xxx flags, which mostly
    // describe the args expected by the native fun
    pub ml_doc: *const c_char, // The __doc__ attribute, or NULL
}

/// The expected end marker for arrays of HPy functions used in module inits.
pub const HPyMethodDef_END: HPyMethodDef = HPyMethodDef {
    ml_name: ptr::null(),
    ml_meth: None,
    ml_flags: 0,
    ml_doc: ptr::null(),
};

#[repr(C)]
pub struct HPyModuleDef {
    _dummy: *const c_void,
    pub m_name: *const c_char,
    pub m_doc: *const c_char,
    pub m_size: HPy_ssize_t,
    pub m_methods: *const HPyMethodDef,
}

/// an empty `HPyModuleDef` that can be used as a placeholder in `static` declarations
pub const HPyModuleDef_INIT: HPyModuleDef = HPyModuleDef {
    _dummy: ptr::null(),
    m_name: ptr::null(),
    m_doc: ptr::null(),
    m_size: -1,
    m_methods: &HPyMethodDef_END as *const _,
};

pub fn HPy_IsNull(h: HPy) -> bool {
    h.0 == 0
}
