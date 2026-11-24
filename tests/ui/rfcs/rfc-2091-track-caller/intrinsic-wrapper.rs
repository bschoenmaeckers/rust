#![feature(location_function)]
//@ run-pass
//@ revisions: default mir-opt
//@[default] compile-flags: -Zinline-mir=no
//@[mir-opt] compile-flags: -Zmir-opt-level=4

macro_rules! caller_location_from_macro {
    () => (core::panic::Location::caller());
}

fn main() {
    let loc = core::panic::Location::caller();
    assert_eq!(loc.file(), file!());
    assert_eq!(loc.line(), 12);
    assert_eq!(loc.column(), 15);
    assert_eq!(loc.function(), Some("main"));

    // `Location::caller()` in a macro should behave similarly to `file!` and `line!`,
    // i.e. point to where the macro was invoked, instead of the macro itself.
    let loc2 = caller_location_from_macro!();
    assert_eq!(loc2.file(), file!());
    assert_eq!(loc2.line(), 20);
    assert_eq!(loc2.column(), 16);
    assert_eq!(loc.function(), Some("main"));
}
