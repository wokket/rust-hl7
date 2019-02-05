//! All the methods exposed to external (.Net, Java) callers.
//! Rust users generally won't need these methods.
use super::*;
use libc::c_char;
use std::ffi::{CStr, CString};

/// Accepts a ptr from any of the methods that return a ```*mut c_char``` and frees the memory associated with the underlying [`String`].
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

/// Accepts a ptr originaly returned from a call to [`build_message`] and frees the memory associated with the underlying object.
#[no_mangle]
pub extern "C" fn free_message(msg_ptr: *mut Message) {
    unsafe {
        if msg_ptr.is_null() {
            return;
        }

        Box::from_raw(msg_ptr); //recreate boxed var, then drop it out of scope to clean
    };
}

/// The main entry point for external callers.  Accepts a ```String``` in C-standard format and parses it into an object representing the [`Message`].
/// NOTE: You **must** subsequently pass the returned pointer into [`free_message`] to ensure the memory is reclaimed, or you will leak the memory!
#[no_mangle]
pub extern "C" fn build_message(s: *const c_char) -> *mut Message {
    // println!("Into build_message...");

    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap().to_string();

    //eprintln!("Building message from string value: {}", r_str);

    let m = message_parser::MessageParser::parse_message(r_str);

    //eprintln!("Message init to: {:?}", m);

    let return_ptr = Box::into_raw(Box::new(m)); //box onto the heap for stability, then get a raw ptr we can pass outside.

    return_ptr
}

#[no_mangle]
pub extern "C" fn get_field(
    ptr: *const Message,
    segment_ptr: *const c_char,
    field_index: usize,
) -> *mut c_char {
    //eprintln!("Into get_field()");

    let obj: &Message = unsafe { &*ptr };

    let segment_cstr = unsafe {
        assert!(!segment_ptr.is_null());
        CStr::from_ptr(segment_ptr)
    };

    let segment_str = segment_cstr.to_str().unwrap();

    let result = obj.get_field(segment_str, field_index);
    //println!("Returning field value: {}", result);

    let c_string = CString::new(result).unwrap();
    c_string.into_raw()
}
