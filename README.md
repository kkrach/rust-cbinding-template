# RUST C-Binding Template

Template for creating RUST projects with code in C

 - [Raw Crate](#raw-crate)
 - [Safe Crate](#safe-crate)
 - [Example Application](#example-application)
 - [Unit Tests](#unit-tests)

## Raw Crate

In this template a small C library (`libfoo.a`) is created. It contains the
following 3 methods:

```c
void foo_reset();
int foo_get_value();
void foo_set_value(int value);
```

The `raw_foo/build.rs` compile the library with `make` and provides with
`bingen` a `bindings.rs`:

```rust
/* automatically generated by rust-bindgen 0.59.1 */

extern "C" {
    pub fn foo_reset();
}
extern "C" {
    pub fn foo_get_value() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn foo_set_value(value: ::std::os::raw::c_int);
}
```

This is purely exported by the `raw_foo` crate and can be used with the
`unsafe` keyword (see example):

```rust
unsafe {
	println!("initial raw value: {}", raw_foo::foo_get_value());
	raw_foo::foo_set_value(13);
	println!("raw value after setting 13: {}", raw_foo::foo_get_value());
	raw_foo::foo_reset();
	println!("raw value after reset: {}", raw_foo::foo_get_value());
}
```

## Safe Crate

The safe crate wraps the unsafe functions of the raw create. It would - if
necessary - also do the conversion from/to RUST types (e.g. vector). In this
simple example this is not necessary. So the wrapper would look like this:

```rust
pub fn foo_set_value(value : i32) {
	unsafe {
		raw_foo::foo_set_value(value);
	}
}
```

## Example Application

The example application uses the two creates.

```rust
unsafe { raw_foo::foo_set_value(13); }
...
safe_foo::foo_set_value(13);
```

## Unit Tests

Each create defines tests for its methods:

```
$ cargo test
   Compiling safe_foo v0.1.0 (/home/charly/Projects/RUST/rust-cbinding-template/safe_foo)
   Compiling example v0.1.0 (/home/charly/Projects/RUST/rust-cbinding-template/example)
    Finished test [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests (target/debug/deps/example-bbc66704e8a894a2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/raw_foo-60c4440c51b6f280)

running 3 tests
test tests::initial ... ok
test tests::reset ... ok
test tests::setter ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/safe_foo-ca0ccc922d825209)

running 3 tests
test tests::initial ... ok
test tests::setter ... ok
test tests::reset ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests raw_foo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests safe_foo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$
```
