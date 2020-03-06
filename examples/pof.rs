use hpy::ffi::HPyContext;
use hpy::ffi::*;
use std::{ptr, slice};

#[no_mangle]
unsafe extern "C" fn do_nothing_trampoline(
    slf: *mut PyObject,
    _noargs: *mut PyObject,
) -> *mut PyObject {
    let ctx = _CTX_FOR_TRAMPOLINES;

    ((*ctx).ctx_CallRealFunctionFromTrampoline)(
        ctx,
        slf,
        ptr::null_mut(),
        ptr::null_mut(),
        do_nothing_impl as *mut c_void,
        HPy_METH_NOARGS,
    )
}

#[no_mangle]
unsafe extern "C" fn do_nothing_impl(ctx: HPyContext, _slf: HPy) -> HPy {
    println!("Rust is happy not to do anything!");
    ((*ctx).ctx_Dup)(ctx, (*ctx).h_None.clone())
}

#[no_mangle]
pub unsafe extern "C" fn do_nothing(
    out_func: *mut *mut c_void,
    // TODO function pointers are not mutable in Rust (and not pointers)
    _out_trampoline: *mut _HPy_CPyCFunction,
) {
    *out_func = do_nothing_impl as *mut c_void;
    *_out_trampoline = do_nothing_trampoline;
}

#[no_mangle]
pub unsafe extern "C" fn add_ints(
    out_func: *mut *mut c_void,
    _out_trampoline: *mut _HPy_CPyCFunction,
) {
    *out_func = add_ints_impl as *mut c_void;
    *_out_trampoline = add_ints_trampoline;
}

#[no_mangle]
unsafe extern "C" fn add_ints_trampoline(slf: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let ctx = _CTX_FOR_TRAMPOLINES;

    ((*ctx).ctx_CallRealFunctionFromTrampoline)(
        ctx,
        slf,
        args,
        ptr::null_mut(),
        add_ints_impl as *mut c_void,
        HPy_METH_VARARGS,
    )
}

#[no_mangle]
unsafe extern "C" fn add_ints_impl(
    ctx: HPyContext,
    _slf: HPy,
    args: *const HPy,
    nargs: HPy_ssize_t,
) -> HPy {
    let args = slice::from_raw_parts(args, nargs as usize);
    if nargs != 2 {
        return HPy_NULL;
    }
    // XXX check for exceptions (same comment in HPyArg_Parse
    // we don't need to make new handles, but still, in Rust we have to clone
    // because HPy is not Copy
    let a = ((*ctx).ctx_Long_AsLong)(ctx, args[0].clone());
    let b = ((*ctx).ctx_Long_AsLong)(ctx, args[1].clone());
    return ((*ctx).ctx_Long_FromLong)(ctx, a + b);
}

pub unsafe extern "C" fn double(
    out_func: *mut *mut c_void,
    _out_trampoline: *mut _HPy_CPyCFunction,
) {
    *out_func = double_impl as *mut c_void;
    *_out_trampoline = double_trampoline;
}

#[no_mangle]
unsafe extern "C" fn double_trampoline(slf: *mut PyObject, arg: *mut PyObject) -> *mut PyObject {
    let ctx = _CTX_FOR_TRAMPOLINES;

    ((*ctx).ctx_CallRealFunctionFromTrampoline)(
        ctx,
        slf,
        arg,
        ptr::null_mut(),
        double_impl as *mut c_void,
        HPy_METH_O,
    )
}


#[no_mangle]
unsafe extern "C" fn double_impl(
    ctx: HPyContext,
    _slf: HPy,
    obj: HPy,
) -> HPy {
    ((*ctx).ctx_Number_Add)(ctx, obj.clone(), obj)
}

static mut _CTX_FOR_TRAMPOLINES: HPyContext = ptr::null_mut();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn HPyInit_pof(ctx: HPyContext) -> HPy {
    unsafe {
        _CTX_FOR_TRAMPOLINES = ctx;
        static mut MODULE_DEF: HPyModuleDef = HPyModuleDef_INIT;
        MODULE_DEF.m_name = "pof\0".as_ptr() as *const c_char;
        static mut METHODS: [HPyMethodDef; 4] = [
            HPyMethodDef {
                ml_name: "do_nothing\0".as_ptr() as *const c_char,
                ml_meth: Some(do_nothing),
                ml_flags: HPy_METH_NOARGS,
                ml_doc: "Really, do nothing\0".as_ptr() as *const c_char,
            },
            HPyMethodDef {
                ml_name: "add_ints\0".as_ptr() as *const c_char,
                ml_meth: Some(add_ints),
                ml_flags: HPy_METH_VARARGS,
                ml_doc: "Lots of infrastructure for an addition, demonstrates HPy_METH_VARARGS\0"
                    .as_ptr() as *const c_char,
            },
            HPyMethodDef {
                ml_name: "double\0".as_ptr() as *const c_char,
                ml_meth: Some(double),
                ml_flags: HPy_METH_O,
                ml_doc: "Demonstrates HPy_METH_O\0"
                    .as_ptr() as *const c_char,
            },
            HPyMethodDef_END,
        ];

        MODULE_DEF.m_methods = METHODS.as_ptr();
        let m = ((*ctx).ctx_Module_Create)(ctx, &mut MODULE_DEF);
        m // pof.c checks HPy_IsNull, only to return NULL
    }
}
