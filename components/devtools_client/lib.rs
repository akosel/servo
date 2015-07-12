/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Servo, the mighty web browser engine from the future.
//
// This is a very simple library that wires all of Servo's components
// together as type `Browser`, along with a generic client
// implementing the `WindowMethods` trait, to create a working web
// browser.
//
// The `Browser` type is responsible for configuring a
// `Constellation`, which does the heavy lifting of coordinating all
// of Servo's internal subsystems, including the `ScriptTask` and the
// `LayoutTask`, as well maintains the navigation context.
//
// The `Browser` is fed events from a generic type that implements the
// `WindowMethods` trait.
