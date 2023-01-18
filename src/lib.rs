/*
* Copyright (c) 2023 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use std::ffi::CString;
use liquidity_check::validate;
use liquidity_check::split;

#[repr(C)]
struct liquidity_check_split_t {
    is_err: c_int,
    is_none: c_int,
    p1: *mut c_char,
    p2: *mut c_char
}

#[no_mangle]
unsafe extern "C" fn liquidity_check_validate(input: *const c_char) -> c_int {
    let input_rs = match CStr::from_ptr(input).to_str() {
	Ok(str) => str,
	Err(_) => return -1
    };
    if validate(input_rs) {
	1
    } else {
	0
    }
}

#[no_mangle]
unsafe extern "C" fn liquidity_check_split(input: *const c_char) -> liquidity_check_split_t {
    let input_rs = match CStr::from_ptr(input).to_str() {
	Ok(str) => str,
	Err(_) => return liquidity_check_split_t { is_err: 1, is_none: 0, p1: std::ptr::null_mut(), p2: std::ptr::null_mut()}
    };
    match split(input_rs) {
	Some((p1, p2)) => {
	    let c_p1 = match CString::new(p1) {
		Ok(cstr) => cstr.into_raw(),
		Err(_) => std::ptr::null_mut()
	    };
	    let c_p2 = match CString::new(p2) {
		Ok(cstr) => cstr.into_raw(),
		Err(_) => std::ptr::null_mut()
	    };
	    liquidity_check_split_t { is_err: 0, is_none: 0, p1: c_p1, p2: c_p2}
	},
	None => return liquidity_check_split_t { is_err: 0, is_none: 1, p1: std::ptr::null_mut(), p2: std::ptr::null_mut()}
    }
}

#[no_mangle]
unsafe extern "C" fn liquidity_check_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
	let _ = CString::from_raw(ptr);
    }
}
