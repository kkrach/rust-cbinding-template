#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn initial() {
        unsafe {
            assert_eq!(foo_get_value(), 42);
        }
    }

    #[test]
    fn setter() {
        unsafe {
            foo_set_value(13);
            assert_eq!(foo_get_value(), 13);
        }
    }

    #[test]
    fn reset() {
        unsafe {
            foo_set_value(13);
            foo_reset();
            assert_eq!(foo_get_value(), 42);
        }
    }
}
