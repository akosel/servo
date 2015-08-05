#![feature(slice_extras)]
#![feature(slice_patterns)]
extern crate util;
//extern crate servo;
//extern crate compositing;
extern crate devtools;
extern crate devtools_client;

mod harness;
#[test]
fn it_works() {
    harness::start_servo();
    harness::start_client();
    assert!(true, true);
}
