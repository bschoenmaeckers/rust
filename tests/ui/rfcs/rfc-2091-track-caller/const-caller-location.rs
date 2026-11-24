#![feature(location_function)]
//@ run-pass
//@ revisions: default mir-opt
//@[mir-opt] compile-flags: -Zmir-opt-level=4

use std::panic::Location;

const LOCATION: &Location = Location::caller();

const TRACKED: &Location = tracked();
#[track_caller]
const fn tracked() -> &'static Location<'static> {
    Location::caller()
}

const NESTED: &Location = nested_location();
const fn nested_location() -> &'static Location<'static> {
    Location::caller()
}

const CONTAINED: &Location = contained();
const fn contained() -> &'static Location<'static> {
    tracked()
}

fn main() {
    assert_eq!(LOCATION.file(), file!());
    assert_eq!(LOCATION.line(), 8);
    assert_eq!(LOCATION.column(), 29);
    assert_eq!(LOCATION.function(), Some("LOCATION"));

    assert_eq!(TRACKED.file(), file!());
    assert_eq!(TRACKED.line(), 10);
    assert_eq!(TRACKED.column(), 28);
    assert_eq!(TRACKED.function(), Some("TRACKED"));

    assert_eq!(NESTED.file(), file!());
    assert_eq!(NESTED.line(), 18);
    assert_eq!(NESTED.column(), 5);
    assert_eq!(NESTED.function(), Some("nested_location"));

    assert_eq!(CONTAINED.file(), file!());
    assert_eq!(CONTAINED.line(), 23);
    assert_eq!(CONTAINED.column(), 5);
    assert_eq!(CONTAINED.function(), Some("contained"));
}
