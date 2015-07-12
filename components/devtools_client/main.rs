/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Devtools Client
//! [glutin]: https://github.com/tomaka/glutin

extern crate rustc_serialize;

use rustc_serialize::{json, Encodable};
use rustc_serialize::json::Json;
use std::io::{self, Read, Write, stdin, stdout};
use std::net::TcpStream;
use std::error::Error;

pub trait JsonPacketStream {
    fn write_json_packet<'a, T: Encodable>(&mut self, obj: &T);
    fn read_json_packet(&mut self) -> io::Result<Option<Json>>;
}

impl JsonPacketStream for TcpStream {
    fn write_json_packet<'a, T: Encodable>(&mut self, obj: &T) {
        let s = json::encode(obj).unwrap().replace("__type__", "type");
        println!("<- {}", s);
        self.write_all(s.len().to_string().as_bytes()).unwrap();
        self.write_all(&[':' as u8]).unwrap();
        self.write_all(s.as_bytes()).unwrap();
    }

    fn read_json_packet<'a>(&mut self) -> io::Result<Option<Json>> {
        // https://wiki.mozilla.org/Remote_Debugging_Protocol_Stream_Transport
        // In short, each JSON packet is [ascii length]:[JSON data of given length]
        let mut buffer = vec!();
        loop {
            let mut buf = [0];
            let byte = match try!(self.read(&mut buf)) {
                0 => return Ok(None),  // EOF
                1 => buf[0],
                _ => unreachable!(),
            };
            match byte {
                b':' => {
                    let packet_len_str = String::from_utf8(buffer).unwrap();
                    let packet_len = u64::from_str_radix(&packet_len_str, 10).unwrap();
                    let mut packet = String::new();
                    self.take(packet_len).read_to_string(&mut packet).unwrap();
                    println!("reading packet {}", packet);
                    return Ok(Some(Json::from_str(&packet).unwrap()))
                },
                c => buffer.push(c),
            }
        }
    }
}

#[derive(RustcEncodable)]
struct ConsoleAPICall {
    from: String,
    to: String,
    __type__: String,
    text: String,
}

fn main() {
    println!("Hello");
    // TODO How to get the port for the devtools listener
    let mut stream = TcpStream::connect("127.0.0.1:6000").unwrap();

    'outer: loop {
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

        print!(">>>");
        io::stdout().flush().unwrap();
        let mut message = String::new();
        io::stdin().read_line(&mut message)
            .ok()
            .expect("Failed to read line");

        println!("Message {}", message);

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
        let msg = ConsoleAPICall {
            from: "test".to_string(),
            to: "console2".to_string(),
            __type__: "evaluateJS".to_string(),
            text: message.trim_right_matches('\n').to_string(),
        };
        stream.write_json_packet(&msg);
    }
}
