// An integration test of a function in the adder crate
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, ch1103::add_two(2))
}