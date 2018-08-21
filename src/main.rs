//! A VM implementation for the Synacor Challenge.

#![feature(range_contains)]

pub mod ops;
mod util;
pub mod vm;

#[macro_use]
extern crate enum_primitive;
extern crate num;

fn main() {
    let mut args = std::env::args();
    args.next(); // Pop the executable name
    let mut filename = match args.next() {
        None => panic!("Must provide binary filename"),
        Some(item) => item,
    };
    let mut vm = vm::VM::from_file(filename.as_mut_str());
    vm.run();
}
