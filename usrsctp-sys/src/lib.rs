#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_finish() {
        unsafe {
            usrsctp_init(0, None, None);
            usrsctp_finish();
        }
    }
}
