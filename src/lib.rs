extern crate libpulse_sys;

use libpulse_sys::*;
use std::ptr::{null, null_mut};
use std::{mem, slice};


struct SimpleClient<Fmt, Channels> {
    simple: *const pa_simple
}

impl<Fmt, Channels> SimpleClient<Fmt, Channels> {
    fn channel_count() -> u8 {
        /* Type vars seem only usable in signatures */
        [0; Channels].len() as u8
    }
    
    fn new(name: &str, desc: &str, dir: pa_stream_direction_t, rate: u32) {
        let ss = pa_sample_spec {
            format: PA_SAMPLE_S16LE,
            channels: Self::channel_count(),
            rate: rate
        };
        let s = unsafe {
            pa_simple_new(null(),             // Use the default server.
                          name.as_ptr() as *const i8,  // Our application's name.
                          dir,
                          null(),             // Use the default device.
                          desc.as_ptr() as *const i8,  // Description of our stream.
                          &ss,                // Our sample format.
                          null(),             // Use default channel map
                          null(),             // Use default buffering attributes.
                          null_mut(),         // Ignore error code.
                         )
        };
    }
    
    // TODO: Drop
}

pub struct Playback<Fmt, Channels> {
    client: SimpleClient<Fmt, Channels>
}

pub impl<Fmt, Channels> SimpleClient<Fmt, Channels> {
    pub fn new(name: &str, desc: &str, rate: u32) {
        Playback {
            client: SimpleClient::new(name, desc, PA_STREAM_PLAYBACK, rate)
        }
    }
    
    pub fn write(data: &[[Fmt; Channels]]) -> std::io::Result<()> {
    }
}


#[test]
fn it_works() {
}
