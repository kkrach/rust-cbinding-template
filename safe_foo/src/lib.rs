extern crate raw_foo;

pub fn foo_read_file(filename: &str, content: &mut Vec<u8>) -> bool {
    let c_filename = std::ffi::CString::new(filename).expect("CString::new failed");
    let buffer = content.as_mut_ptr() as *mut _;
    let mut buflen: raw_foo::size_t = content.len() as _;

    unsafe {
        let result = raw_foo::foo_read_file(c_filename.as_ptr(), buffer, &mut buflen);
        match result {
            raw_foo::FOO_Error_FOO_INVALID_ARGUMENT => {
                println!("Error: Internal error!");
                content.resize(0, 0);
                false
            }
            raw_foo::FOO_Error_FOO_FILE_NOT_FOUND => {
                println!("Error: File '{}' not found!", filename);
                content.resize(0, 0);
                false
            }
            raw_foo::FOO_Error_FOO_ERROR_READ => {
                println!("Error: Failed reading file {}!", filename);
                content.resize(0, 0);
                false
            }
            raw_foo::FOO_Error_FOO_OK => {
                content.resize(buflen as usize, 0);
                true
            }
            _ => {
                println!("Error: Unknown error when reading {}!", filename);
                content.resize(0, 0);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn file_not_found() {
        let mut content: Vec<u8> = vec![0; 1024];
        assert_eq!(foo_read_file("sdflskjlkjsdfljösdf", &mut content), false);
        assert_eq!(content.len(), 0);
    }

    #[test]
    fn zero_buffer() {
        let mut content: Vec<u8> = vec![0; 0];
        assert_eq!(foo_read_file("Caro.toml", &mut content), false);
        assert_eq!(content.len(), 0);
    }

    #[test]
    fn file_found() {
        let mut content: Vec<u8> = vec![0; 1024];
        assert_eq!(foo_read_file("Cargo.toml", &mut content), true);
        assert_ne!(content.len(), 0);
    }
}
