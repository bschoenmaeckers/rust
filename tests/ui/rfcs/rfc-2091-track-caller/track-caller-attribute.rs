#![feature(location_function)]
//@ run-pass
//@ revisions: default mir-opt
//@[mir-opt] compile-flags: -Zmir-opt-level=4

use std::panic::Location;

#[track_caller]
fn tracked() -> &'static Location<'static> {
    Location::caller()
}

fn nested_intrinsic() -> &'static Location<'static> {
    Location::caller()
}

fn nested_tracked() -> &'static Location<'static> {
    tracked()
}

fn main() {
    let location = Location::caller();
    assert_eq!(location.file(), file!());
    assert_eq!(location.line(), 22);
    assert_eq!(location.column(), 20);
    assert_eq!(location.function(), Some("main"));

    let tracked = tracked();
    assert_eq!(tracked.file(), file!());
    assert_eq!(tracked.line(), 28);
    assert_eq!(tracked.column(), 19);
    assert_eq!(tracked.function(), Some("main"));

    let nested = nested_intrinsic();
    assert_eq!(nested.file(), file!());
    assert_eq!(nested.line(), 14);
    assert_eq!(nested.column(), 5);
    assert_eq!(nested.function(), Some("nested_intrinsic"));

    let contained = nested_tracked();
    assert_eq!(contained.file(), file!());
    assert_eq!(contained.line(), 18);
    assert_eq!(contained.column(), 5);
    assert_eq!(contained.function(), Some("nested_tracked"));
}
