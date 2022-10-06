mod ck3;

use std::ffi::c_void;

struct CtxWrapper {
    ctx: *mut c_void,
}

impl CtxWrapper {
    fn new(ctx: *mut c_void) -> Self {
        CtxWrapper { ctx }
    }
}

unsafe impl Send for CtxWrapper {}
