mod bindings;

use std::ffi::CString;
use bindings::{cool_function, CoolStruct};
// extern { fn hello(); }
fn main() {
    println!("Hello, world!");

    unsafe {
        let mut cool_struct = CoolStruct {
            x: 1,
            y: 911,
        };
        let s = CString::new("data data data data").expect("CString::new failed");
        cool_function(122, s.into_raw() as i8 , &mut cool_struct);
        // hello();
    }
}
