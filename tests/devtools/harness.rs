/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use devtools_client::run_client;
use std::thread;
use std::env;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender, RecvError};

//fn run_servo() {
//    let port: u16 = 6000;
//    let mut opts = opts::default_opts();
//    opts.headless = true;
//    opts.devtools_port = Some(port);
//
//    let window = if opts::get().headless {
//        None
//    } else {
//        Some(app::create_window(std::ptr::null_mut()))
//    };
//
//    let mut browser = BrowserWrapper {
//        browser: Browser::new(window.clone()),
//    };
//
//    browser.browser.handle_events(vec![WindowEvent::InitializeCompositing]);
//
//    println!("Running browser instance");
//    // Feed events from the window to the browser until the browser
//    // says to stop.
//    loop {
//        let should_continue = match window {
//            None => browser.browser.handle_events(Vec::new()),
//            Some(ref window) => browser.browser.handle_events(window.wait_events()),
//        };
//        if !should_continue {
//            break
//        }
//    };
//
//    let BrowserWrapper {
//        browser
//    } = browser;
//    browser.shutdown();
//}
//
//struct BrowserWrapper {
//    browser: Browser,
//}
//
//impl app::NestedEventLoopListener for BrowserWrapper {
//    fn handle_event_from_nested_event_loop(&mut self, event: WindowEvent) -> bool {
//        let is_resize = match event {
//            WindowEvent::Resize(..) => true,
//            _ => false,
//        };
//        if !self.browser.handle_events(vec![event]) {
//            return false
//        }
//        if is_resize {
//            self.browser.repaint_synchronously()
//        }
//        true
//    }
//}

fn servo_path() -> PathBuf {
    let current_exe = env::current_exe().ok().expect("Could not locate current executable");
    current_exe.parent().unwrap().join("servo")
}

pub fn start_servo() -> Sender<String> {
    let (sender, receiver) = channel();
    {
        let sender = sender.clone();
        let handle = thread::spawn(move || {
            println!("I can run!");
            let args: Vec<String> = env::args().collect();
            let mut parts = args.tail().split(|e| &**e == "--");
            let servo_args = ["http://google.com", "--devtools", "--headless"];
            println!("Servo args {:?}", servo_args);
            let output = match Command::new(&servo_path()).args(&servo_args).spawn() {
                Ok(p) => p,
                Err(e) => panic!("failed to execute process: {}", e),
            };
            println!("Beyond output");
            //let stderr = String::from_utf8(output.stderr).unwrap();
            //let stdout = String::from_utf8(output.stdout).unwrap();
            //println!("stdout {}", stdout);

            //if stderr.contains("Unrecognized") {
            //    println!("Servo: {}", stderr);
            //}
        }).join();
    }
    sender
}

pub fn start_client() -> Sender<String> {
    let (sender, receiver) = channel();
    {
        let sender = sender.clone();
        let handle = thread::spawn(move || {
            run_client();
        }).join();
    }
    sender
}
