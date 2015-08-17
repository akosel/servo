/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use devtools_client::start_client;
use std::thread;
use std::env;
use std::process::{Child, Command};
use std::path::{PathBuf};

fn servo_path() -> PathBuf {
    let current_exe = env::current_exe().ok().expect("Could not locate current executable");
    current_exe.parent().unwrap().join("servo")
}

pub fn start_servo() -> Child {
    let servo_args = ["https://google.com", "--devtools", "--headless"];
    println!("Servo args {:?}", servo_args);
    let mut child = Command::new(&servo_path())
                                .args(&servo_args)
                                .spawn() 
                                .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

    //let mut ecode = child.wait()
    //     .unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });
    child
}
