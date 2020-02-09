// use std::slice::from_raw_parts;
// use std::os::raw::c_char;
// use std::ffi::{ CString, CStr };

// pub fn test_fn(a:i32, b:i32) -> i32 {
//     a + b
// }

#[no_mangle]
pub extern "C" fn rust_add(a:i32, b:i32) -> i32 {
    a + b
}

// #[no_mangle]
// pub extern "C" fn rust_sum(ptr: *const i32, length: usize) -> i32 {
//     let nums = unsafe {
//         from_raw_parts(ptr as *const i32, length)
//     };

//     nums.iter().sum()
// }

// #[no_mangle]
// pub extern "C" fn rust_add_prefix(text: *const c_char) -> *const c_char {
//     let str_bytes = unsafe {
//         CStr::from_ptr(text).to_bytes()
//     };
//     let text = std::str::from_utf8(str_bytes).expect("from utf8 string");
//     let mut new_text = "rust_prefix_".to_string();
//     new_text.push_str(text);

//     CString::new(new_text).unwrap().into_raw()
// }

// #[repr(C)]
// enum Hobby {
//     SWIM,
//     READ,
//     EAT,
// }

// struct Student {
//     name: String,
//     age: u32,
//     hobby: Vec<Hobby>
// }

// #[repr(C)]
// struct StudentFFI {
//     name: *const char,
//     age: u32,
//     hobby: VecFFI<Hobby>
// }

// #[repr(C)]
// pub struct VecFFI<T> {
//     pub ptr: *const T,
//     pub len: usize
// }