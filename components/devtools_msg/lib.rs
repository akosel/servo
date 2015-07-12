/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! This module contains messages pulled from devtools/actors. Creating a separate
//! crate allows for these messages to be used by other modules.

#![crate_name = "devtools_msg"]
#![crate_type = "rlib"]

#![allow(non_snake_case)]

extern crate devtools_traits;
extern crate rustc_serialize;
extern crate time;

use rustc_serialize::{Encodable, Encoder};
use rustc_serialize::json::{self, Json};

use time::PreciseTime;

/// Console Actor messages
#[derive(RustcEncodable)]
pub struct StartedListenersTraits {
    pub customNetworkRequest: bool,
}

#[derive(RustcEncodable)]
pub struct StartedListenersReply {
    pub from: String,
    pub nativeConsoleAPI: bool,
    pub startedListeners: Vec<String>,
    pub traits: StartedListenersTraits,
}

#[derive(RustcEncodable)]
pub struct GetCachedMessagesReply {
    pub from: String,
    pub messages: Vec<json::Object>,
}

#[derive(RustcEncodable)]
pub struct StopListenersReply {
    pub from: String,
    pub stoppedListeners: Vec<String>,
}

#[derive(RustcEncodable)]
pub struct AutocompleteReply {
    pub from: String,
    pub matches: Vec<String>,
    pub matchProp: String,
}

#[derive(RustcEncodable)]
pub struct EvaluateJSReply {
    pub from: String,
    pub input: String,
    pub result: Json,
    pub timestamp: u64,
    pub exception: Json,
    pub exceptionMessage: String,
    pub helperResult: Json,
}

/// Inspector Actor Messages 
#[derive(RustcEncodable)]
pub struct GetHighlighterReply {
    pub highligter: HighlighterMsg, // sic.
    pub from: String,
}

#[derive(RustcEncodable)]
pub struct HighlighterMsg {
    pub actor: String,
}

#[derive(RustcEncodable)]
pub struct ShowBoxModelReply {
    pub from: String,
}

#[derive(RustcEncodable)]
pub struct HideBoxModelReply {
    pub from: String,
}

/// Memory Actor Messages 
#[derive(RustcEncodable)]
pub struct TimelineMemoryReply {
    pub jsObjectSize: u64,
    pub jsStringSize: u64,
    pub jsOtherSize: u64,
    pub domSize: u64,
    pub styleSize: u64,
    pub otherSize: u64,
    pub totalSize: u64,
    pub jsMilliseconds: f64,
    pub nonJSMilliseconds: f64,
}

/// Network Event Actor Messages 
#[derive(RustcEncodable)]
pub struct ResponseStartMsg {
    pub httpVersion: String,
    pub remoteAddress: String,
    pub remotePort: u32,
    pub status: String,
    pub statusText: String,
    pub headersSize: u32,
    pub discardResponseBody: bool,
}

#[derive(RustcEncodable)]
pub struct GetRequestHeadersReply {
    pub from: String,
    pub headers: Vec<String>,
    pub headerSize: u8,
    pub rawHeaders: String
}

/// Root Actor Messages
#[derive(RustcEncodable)]
pub struct ActorTraits {
    pub sources: bool,
    pub highlightable: bool,
    pub customHighlighters: Vec<String>,
}

#[derive(RustcEncodable)]
pub struct ErrorReply {
    pub from: String,
    pub error: String,
    pub message: String,
}

#[derive(RustcEncodable)]
pub struct ListTabsReply {
    pub from: String,
    pub selected: u32,
    pub tabs: Vec<TabActorMsg>,
}

#[derive(RustcEncodable)]
pub struct RootActorMsg {
    pub from: String,
    pub applicationType: String,
    pub traits: ActorTraits,
}

/// Tab Actor Messages 
#[derive(RustcEncodable)]
pub struct TabTraits;

#[derive(RustcEncodable)]
pub struct TabAttachedReply {
    pub from: String,
    pub __type__: String,
    pub threadActor: String,
    pub cacheDisabled: bool,
    pub javascriptEnabled: bool,
    pub traits: TabTraits,
}

#[derive(RustcEncodable)]
pub struct TabDetachedReply {
    pub from: String,
    pub __type__: String,
}

#[derive(RustcEncodable)]
pub struct ReconfigureReply {
    pub from: String
}

#[derive(RustcEncodable)]
pub struct ListFramesReply {
    pub from: String,
    pub frames: Vec<FrameMsg>,
}

#[derive(RustcEncodable)]
pub struct FrameMsg {
    pub id: u32,
    pub url: String,
    pub title: String,
    pub parentID: u32,
}

#[derive(RustcEncodable)]
pub struct TabActorMsg {
    pub actor: String,
    pub title: String,
    pub url: String,
    pub outerWindowID: u32,
    pub consoleActor: String,
    pub inspectorActor: String,
    pub timelineActor: String,
}

/// Timeline Actor Messages
/// XXX Included HighResolutionStamp because it used by several messages, but I feel that 
/// may not be the best way
/// HighResolutionStamp is pub struct that contains duration in milliseconds
/// with accuracy to microsecond that shows how much time has passed since
/// actor registry inited
/// analog https://w3c.github.io/hr-time/#sec-DOMHighResTimeStamp
pub struct HighResolutionStamp(f64);

impl HighResolutionStamp {
    pub fn new(start_stamp: PreciseTime, time: PreciseTime) -> HighResolutionStamp {
        let duration = start_stamp.to(time).num_microseconds()
                                  .expect("Too big duration in microseconds");
        HighResolutionStamp(duration as f64 / 1000 as f64)
    }

    pub fn wrap(time: f64) -> HighResolutionStamp {
        HighResolutionStamp(time)
    }
}

impl Encodable for HighResolutionStamp {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}
#[derive(RustcEncodable)]
pub struct IsRecordingReply {
    pub from: String,
    pub value: bool
}

#[derive(RustcEncodable)]
pub struct StartReply {
    pub from: String,
    pub value: HighResolutionStamp,
}

#[derive(RustcEncodable)]
pub struct StopReply {
    pub from: String,
    pub value: HighResolutionStamp,
}

#[derive(RustcEncodable)]
pub struct TimelineMarkerReply {
    pub name: String,
    pub start: HighResolutionStamp,
    pub end: HighResolutionStamp,
    pub stack: Option<Vec<()>>,
    pub endStack: Option<Vec<()>>,
}

#[derive(RustcEncodable)]
pub struct MarkersEmitterReply {
    pub __type__: String,
    pub markers: Vec<TimelineMarkerReply>,
    pub from: String,
    pub endTime: HighResolutionStamp,
}

#[derive(RustcEncodable)]
pub struct MemoryEmitterReply {
    pub __type__: String,
    pub from: String,
    pub delta: HighResolutionStamp,
    pub measurement: TimelineMemoryReply,
}

#[derive(RustcEncodable)]
pub struct FramerateEmitterReply {
    pub __type__: String,
    pub from: String,
    pub delta: HighResolutionStamp,
    pub timestamps: Vec<HighResolutionStamp>,
}
