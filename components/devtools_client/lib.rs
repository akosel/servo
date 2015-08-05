/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Devtools Client
//! [glutin]: https://github.com/tomaka/glutin

#![crate_name = "devtools_client"]
#![crate_type = "rlib"]

extern crate rustc_serialize;
extern crate devtools_msg;
extern crate time;

use std::io::{self, Write, stdin, stdout};
use std::net::{TcpStream, Shutdown};
use std::error::Error;

use devtools_msg::{ClientAPICall, ConsoleAPICall, ConsoleMsg};
use devtools_msg::protocol::JsonPacketStream;

use time::precise_time_ns;

pub fn run_client() {
    println!("Hello from devtools");
    let port: u16 = 6000;
    let mut stream = TcpStream::connect(&("127.0.0.1", port)).unwrap();
    println!("Connected");

    'outer: loop {
        println!("In loop");
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
        println!("Further down the loop too");
        match stream.read_json_packet() {
            Ok(Some(json_packet)) => {

                println!("json obj {}", json_packet);
            }
            Ok(None) => {
                println!("error: EOF");
                break 'outer
            }
            Err(e) => {
                println!("error: {}", e.description());
                break 'outer
            }
        }

        let _ = stream.shutdown(Shutdown::Both);
        break 'outer;
        print!(">>>");
        //io::stdout().flush().unwrap();
        //let mut message = String::new();
        //io::stdin().read_line(&mut message)
        //    .ok()
        //    .expect("Failed to read line");

        //println!("Message {}", message);

        //let msg = ConsoleAPICall {
        //    from: "test".to_string(),
        //    to: "root".to_string(),
        //    __type__: "listTabs".to_string(),
        //    message: message.trim_right_matches('\n').to_string(),
        //};
        // Common Pattern
        // 1. Get the lay of the land
        // { to: "root".to_string(), __type__: "listAddons".to_string() }
        // { to: "root".to_string(), __type__: "listTabs".to_string() }
        // 2. Attach to a tab (based on above response, presumably)
        // { to: "tabN".to_string(), __type__: "attach".to_string() }
    }
}
