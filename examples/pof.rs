use hpy::ffi::HPyContext;
use hpy::ffi::*;
use std::ptr;

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

static mut _CTX_FOR_TRAMPOLINES: HPyContext = ptr::null_mut();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn HPyInit_pof(ctx: HPyContext) -> HPy {
    unsafe {
        _CTX_FOR_TRAMPOLINES = ctx;
        static mut MODULE_DEF: HPyModuleDef = HPyModuleDef_INIT;
        MODULE_DEF.m_name = "pof\0".as_ptr() as *const c_char;
        static mut METHODS: [HPyMethodDef; 2] = [
            HPyMethodDef {
                ml_name: "do_nothing\0".as_ptr() as *const c_char,
                ml_meth: Some(do_nothing),
                ml_flags: HPy_METH_NOARGS,
                ml_doc: "Really, do nothing\0".as_ptr() as *const c_char,
            },
            HPyMethodDef_END,
        ];

        MODULE_DEF.m_methods = METHODS.as_ptr();
        let m = ((*ctx).ctx_Module_Create)(ctx, &mut MODULE_DEF);
        m // pof.c checks HPy_IsNull, only to return NULL
    }
}
