#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn invalid_input() {
        let filename = std::ffi::CString::new("").expect("CString::new failed");
        let mut content: Vec<u8> = vec![0; 1024];
        let buf = content.as_mut_ptr() as *mut _;
        let mut buflen: size_t = 0;
        unsafe {
            assert_eq!(
                foo_read_file(filename.as_ptr(), buf, &mut buflen),
                FOO_Error_FOO_INVALID_ARGUMENT
            );
        }
        assert_eq!(buflen, 0);
    }

    #[test]
    fn file_not_found() {
        let filename = std::ffi::CString::new("lkjlksjdlfjksdfl").expect("CString::new failed");
        let mut content: Vec<u8> = vec![0; 1024];
        let buf = content.as_mut_ptr() as *mut _;
        let mut buflen: size_t = content.len() as _;
        unsafe {
            assert_eq!(
                foo_read_file(filename.as_ptr(), buf, &mut buflen),
                FOO_Error_FOO_FILE_NOT_FOUND
            );
        }
        assert_eq!(buflen, 0);
    }

    #[test]
    fn file_found() {
        let filename = std::ffi::CString::new("Cargo.toml").expect("CString::new failed");
        let mut content: Vec<u8> = vec![0; 1024];
        let buf = content.as_mut_ptr() as *mut _;
        let mut buflen: size_t = content.len() as _;
        unsafe {
            assert_eq!(
                foo_read_file(filename.as_ptr(), buf, &mut buflen),
                FOO_Error_FOO_OK
            );
        }
        assert_ne!(buflen, 0);
    }
}
