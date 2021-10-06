use crate::bindings::{mrb_close, mrb_load_string, mrb_open, mrb_str_to_cstr};
use std::ffi::{CStr, CString};

#[macro_export]
macro_rules! eval_to {
    ($type:ty, $code:expr) => {
        {
            let result = eval_to_string($code).unwrap_or("".to_string());
            result.parse::<$type>()
        }
    }
}

pub fn eval_to_string(user_code: &str) -> Option<String> {
    let mut code = String::from("(");

    code.push_str(user_code);

    code.push_str(").to_s");

    let code = CString::new(code).expect("Invalid input string");

    let result: String;

    unsafe {
        let mrb = mrb_open();

        let value = mrb_load_string(mrb, code.as_ptr());

        let raw_result = mrb_str_to_cstr(mrb, value);

        result = CStr::from_ptr(raw_result)
            .to_str()
            .expect("Invalid convertion from C string to Rust string")
            .to_string();

        mrb_close(mrb);
    }

    if result == "" {
        None
    } else {
        Some(result)
    }
}
