/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(slice_patterns)]
extern crate devtools;
extern crate devtools_msg;
extern crate devtools_client;
extern crate time;

use devtools_client::start_client;
use devtools_msg::{ClientAPICall, ConsoleAPICall, ConsoleMsg};
use devtools_msg::protocol::JsonPacketStream;

use time::precise_time_ns;

use std::net::{Shutdown};

mod harness;
//#[test]
//fn server_works() {
//    let mut child = harness::start_servo();
//    let result = child.kill();
//    println!("{:?}", result);
//    assert!(result.is_ok());
//}

#[test]
fn start_server() {
    let mut child = harness::start_servo();
    let ecode = child.wait()
         .unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });

    let _result = child.kill();
    assert!(ecode.success());
}

#[test]
fn client_works() {
    let stream = start_client();
    assert_eq!(stream.peer_addr().unwrap().port(), 6000);

    let result = stream.shutdown(Shutdown::Both);
    assert!(result.is_ok());
}

#[test]
fn another_client_works() {
    let stream = start_client();
    assert_eq!(stream.peer_addr().unwrap().port(), 6000);

    let result = stream.shutdown(Shutdown::Both);
    assert!(result.is_ok());
}

#[test]
fn test_sending_packet() {
    let mut stream = start_client();

    let console_msg = ConsoleMsg {
        level: "info".to_string(),
        timeStamp: precise_time_ns(),
        arguments: vec!("foo".to_string()),
        filename: "test".to_string(),
        lineNumber: 10,
        columnNumber: 2
    };
    let msg = ClientAPICall {
        to: "root".to_string(),
        __type__: "listTabs".to_string(),
        message: console_msg,
    };
    stream.write_json_packet(&msg);

    //let result = stream.shutdown(Shutdown::Both);
    assert!(true);
}
