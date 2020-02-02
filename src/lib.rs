use ecl_sys as sys;
use std::{env, ffi::CString, ptr};

/// Embeddable Common-Lisp Runtime
pub struct Runtime;

impl Runtime {
    /// Initialize and boot a new ECL Runtime.
    pub fn new() -> Self {
        let runtime = Self {};
        boot();
        return runtime;
    }

    /// Eval
    pub fn eval<S: Into<String>>(&self, code: S) -> Result<(), ()> {
        let _ = eval(code);
        return Ok(());
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            sys::cl_shutdown();
        }
    }
}

fn eval<S: Into<String>>(code: S) -> *mut sys::cl_lispunion {
    let to_run = CString::new(code.into()).unwrap();
    let to_run_len = to_run.as_bytes().len();

    unsafe {
        let code = sys::si_string_to_object(
            1,
            sys::ecl_make_simple_base_string(to_run.as_ptr() as _, to_run_len as _),
        );
        return sys::cl_eval(code);
    }
}

fn boot() {
    let args = env::args()
        .map(CString::new)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut c_args = args.iter().map(|str| str.as_ptr()).collect::<Vec<_>>();
    c_args.push(ptr::null_mut());
    unsafe {
        sys::cl_boot((c_args.len() - 1) as _, c_args.as_ptr() as _);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime() {
        let runtime = Runtime::new();
        let result = runtime.eval("(+ 1 2)");
        result.unwrap();
    }
}
