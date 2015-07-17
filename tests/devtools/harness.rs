/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
extern crate util;
extern crate servo;
extern crate compositing;
extern crate devtools;
extern crate devtools_client;
// The window backed by glutin
extern crate glutin_app as app;

use util::opts;
use servo::Browser;
use compositing::windowing::WindowEvent;
use devtools_client::run_client;

fn run_servo() {
    let port: u16 = 6000;
    let mut opts = opts::default_opts();
    opts.headless = true;
    opts.devtools_port = Some(port);

    let window = if opts::get().headless {
        None
    } else {
        Some(app::create_window(std::ptr::null_mut()))
    };

    let mut browser = BrowserWrapper {
        browser: Browser::new(window.clone()),
    };

    browser.browser.handle_events(vec![WindowEvent::InitializeCompositing]);

    // Feed events from the window to the browser until the browser
    // says to stop.
    loop {
        let should_continue = match window {
            None => browser.browser.handle_events(Vec::new()),
            Some(ref window) => browser.browser.handle_events(window.wait_events()),
        };
        if !should_continue {
            break
        }
    };

    let BrowserWrapper {
        browser
    } = browser;
    browser.shutdown();
}

struct BrowserWrapper {
    browser: Browser,
}

impl app::NestedEventLoopListener for BrowserWrapper {
    fn handle_event_from_nested_event_loop(&mut self, event: WindowEvent) -> bool {
        let is_resize = match event {
            WindowEvent::Resize(..) => true,
            _ => false,
        };
        if !self.browser.handle_events(vec![event]) {
            return false
        }
        if is_resize {
            self.browser.repaint_synchronously()
        }
        true
    }
}

#[test]
fn it_works() {
    run_servo();
    run_client();
    assert!(true, true);
}
