
#![no_std]

extern crate raw_foo;

pub fn foo_reset() {
	unsafe {
		raw_foo::foo_reset();
	}
}

pub fn foo_get_value() -> i32 {
	unsafe {
		raw_foo::foo_get_value()
	}
}

pub fn foo_set_value(value : i32) {
	unsafe {
		raw_foo::foo_set_value(value);
	}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn initial() {
    	assert_eq!(foo_get_value(), 42);
    }

    #[test]
    fn setter() {
          foo_set_value(13);
        assert_eq!(foo_get_value(), 13);
    }

    #[test]
    fn reset() {
        foo_set_value(13);
        foo_reset();
        assert_eq!(foo_get_value(), 42);
    }
}
