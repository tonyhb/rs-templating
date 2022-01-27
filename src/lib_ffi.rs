use std::ffi::CStr;
use crate::rs_templating;


// Create a function which parses all variables in a template, returning them
// as a single c string joined with a ",".  C strings are easy to read in any
// language;  it's easier than creating our own buffer structs or returning
// c arrays to read.
#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn variables(ctpl: *const libc::c_char) -> *mut libc::c_char {
    if ctpl.is_null() {
        return std::ffi::CString::new("").unwrap().into_raw();
    }

    let buf = unsafe { CStr::from_ptr(ctpl).to_bytes() };

    let tplstr = String::from_utf8(buf.to_vec()).unwrap();

    let tpl = rs_templating::Template::init(tplstr).unwrap_or_else(|_|
        rs_templating::Template::init("".into()).unwrap()
    );

    let vars = tpl.get_variables();
    return std::ffi::CString::new(vars.join(",")).unwrap().into_raw();
}


// Create an "execute" function which accepts a template and JSON string
// as C strings, then executes the function and returns a new string.
#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn execute(ctpl: *const libc::c_char, cjson: *const libc::c_char) -> *mut libc::c_char {

    if ctpl.is_null() {
        return std::ffi::CString::new("").unwrap().into_raw();
    }
    let buf = unsafe { CStr::from_ptr(ctpl).to_bytes() };
    let tplstr = String::from_utf8(buf.to_vec()).unwrap();

    let jsonstr: String = match cjson.is_null() {
        true => "{}".to_string(),
        false => {
            let buf = unsafe { CStr::from_ptr(cjson).to_bytes() };
            String::from_utf8(buf.to_vec()).unwrap()
        },
    };

    let result = match rs_templating::compile_and_execute(tplstr, jsonstr) {
        Ok(str) => str,
        Err(e) => format!("error: {}", e),
    };

    return std::ffi::CString::new(result).unwrap().into_raw();
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn release(ptr: *mut libc::c_char) {
    unsafe { drop(std::ffi::CString::from_raw(ptr)); }
}
