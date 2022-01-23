
extern crate raw_foo;

//use raw_foo::*;
//use safe_foo::*;

fn main() {
    unsafe {
        println!("initial raw value: {}", raw_foo::foo_get_value());
        raw_foo::foo_set_value(13);
        println!("raw value after setting 13: {}", raw_foo::foo_get_value());
        raw_foo::foo_reset();
        println!("raw value after reset: {}", raw_foo::foo_get_value());
    }

    println!("initial safe value: {}", safe_foo::foo_get_value());
    safe_foo::foo_set_value(13);
    println!("safe value after setting 13: {}", safe_foo::foo_get_value());
    safe_foo::foo_reset();
    println!("safe value after reset: {}", safe_foo::foo_get_value());
}
