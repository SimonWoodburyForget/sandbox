// #![feature(main)]

// #[main]
// // const start: &'static Fn(isize, *const *const u8) -> isize = &|_argc, _argv| println!("Hello World!");

#![no_main]
#[no_mangle]
pub static main: &'static (dyn FnMut() + Sync) = &|| println!("Hello world!");
