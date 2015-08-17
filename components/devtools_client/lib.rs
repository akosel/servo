/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Devtools Client

#![crate_name = "devtools_client"]
#![crate_type = "rlib"]

extern crate rustc_serialize;
extern crate devtools_msg;
extern crate devtools_traits;
extern crate time;

use std::net::{TcpStream, Shutdown};
use std::error::Error;
use std::thread;
use std::sync::mpsc::{channel, Sender};

use devtools_msg::{ClientAPICall, ConsoleAPICall, ConsoleMsg};
use devtools_msg::protocol::JsonPacketStream;

use time::precise_time_ns;

pub fn send_msg(mut stream: TcpStream) {
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
}

pub fn start_client(server_killer: Sender<Result<i32, i32>>) -> TcpStream {
    let (sender, receiver) = channel();
    {
        let sender = sender.clone();
        thread::spawn(move || {
            println!("Start client");
            run_client(sender);
            println!("Done with client");
            server_killer.send(Ok(1));
        });
    }
    loop {
        match receiver.recv() {
            Ok(stream) => {
                return stream.try_clone().unwrap();
            }
            Err(e) =>  {
                panic!("Error connecting client: {}", e);
            }
        }
    }
}

fn run_client(sender: Sender<TcpStream>) {
    println!("Hello from devtools");
    let port: u16 = 6000;
    let mut stream = TcpStream::connect(&("127.0.0.1", port));
    while stream.is_err() {
        stream = TcpStream::connect(&("127.0.0.1", port));
    }
    let mut stream_unwrap = stream.unwrap();
    println!("Connected at {:?}", stream_unwrap.peer_addr().unwrap());
    let _result = sender.send(stream_unwrap.try_clone().unwrap());

    'outer: loop {
        println!("In loop");
        match stream_unwrap.read_json_packet() {
            Ok(Some(json_packet)) => {

                println!("client received json obj {}", json_packet);
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
        stream_unwrap.shutdown(Shutdown::Both);
        println!("Further down the loop too");
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
