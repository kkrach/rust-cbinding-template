
extern crate raw_foo;

pub fn foo_read_file(filename :&str, content : &mut [u8]) -> bool {
	let c_filename: *const ::std::os::raw::c_char = filename.as_ptr() as *const ::std::os::raw::c_char;
	let buf = content.as_mut_ptr() as *mut _;
	let buflen = content.len() as _;

	unsafe {
		let result = raw_foo::foo_read_file(c_filename, buf, buflen);
		match result {
			raw_foo::FOO_Error_FOO_FILE_NOT_FOUND => {
				println!("Error: File {} not found!", filename);
				false
			},
			raw_foo::FOO_Error_FOO_ERROR_READ => {
				println!("Error: Failed reading file {}!", filename);
				false
			},
			raw_foo::FOO_Error_FOO_OK => true,
			_ => {
				println!("Error: Unknown error when reading {}!", filename);
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
    fn error() {
		let c_filename = CString::new(b"lkjlksjdlfjksdfl" as &[u8]).unwrap();
		let mut content : Vec<u8> = vec![0; 1024];
    	assert_eq!(foo_read_file(c_filename, &mut content), false);
    }
}
