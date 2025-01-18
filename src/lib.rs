#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn try_ffi() {
    let ctx = DltContext {
        contextID: ['a' as i8, 'b' as i8, 'c' as i8, 'd' as i8],
        log_level_pos: 0,
        log_level_ptr: std::ptr::null_mut(),
        trace_status_ptr: std::ptr::null_mut(),
        mcnt: 0,
    };
    println!("context: {:?}", ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_ffi_test() {
        try_ffi();
    }
}
