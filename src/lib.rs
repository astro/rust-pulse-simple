extern crate libpulse_sys;
extern crate libc;

use libpulse_sys::*;
use std::ptr::{null, null_mut};
use std::mem::{transmute, size_of};
use std::marker::PhantomData;
use std::ffi::CString;


pub trait Sampleable {
    fn format() -> pa_sample_format_t;
}

impl Sampleable for u8 {
    fn format() -> pa_sample_format_t {
        PA_SAMPLE_U8
    }
}

impl Sampleable for i16 {
    fn format() -> pa_sample_format_t {
        if cfg!(target_endian = "little") {
            PA_SAMPLE_S16LE
        } else {
            PA_SAMPLE_S16BE
        }
    }
}

impl Sampleable for f32 {
    fn format() -> pa_sample_format_t {
        if cfg!(target_endian = "little") {
            PA_SAMPLE_FLOAT32LE
        } else {
            PA_SAMPLE_FLOAT32BE
        }
    }
}

impl Sampleable for i32 {
    fn format() -> pa_sample_format_t {
        if cfg!(target_endian = "little") {
            PA_SAMPLE_S32LE
        } else {
            PA_SAMPLE_S32BE
        }
    }
}

pub trait ChannelCount {
    fn count() -> u8;

    type S: Sampleable;
    fn format() -> pa_sample_format_t {
        Self::S::format()
    }
    fn sample_size() -> usize {
        Self::count() as usize * size_of::<Self::S>()
    }
}

impl<S> ChannelCount for [S; 1] where S: Sampleable {
    type S = S;
    fn count() -> u8 {
        1
    }
}

impl<S> ChannelCount for [S; 2] where S: Sampleable {
    type S = S;
    fn count() -> u8 {
        2
    }
}

struct SimpleClient<C: ChannelCount> {
    simple: *mut pa_simple,
    phantom: PhantomData<C>,
}

impl<C: ChannelCount> SimpleClient<C> {
    fn new(name: &str, desc: &str, dir: pa_stream_direction_t, rate: u32) -> Self {
        let ss = pa_sample_spec {
            format: C::format(),
            channels: C::count(),
            rate: rate
        };
        let name_c = CString::new(name).unwrap();
        let desc_c = CString::new(desc).unwrap();
        let s = unsafe {
            pa_simple_new(null(),             // Use the default server.
                          name_c.as_ptr() as *const i8,  // Our application's name.
                          dir,
                          null(),             // Use the default device.
                          desc_c.as_ptr() as *const i8,  // Description of our stream.
                          &ss,                // Our sample format.
                          null(),             // Use default channel map
                          null(),             // Use default buffering attributes.
                          null_mut(),         // Ignore error code.
                         )
        };
        assert!(s != null_mut());
        SimpleClient {
            simple: s,
            phantom: PhantomData
        }
    }
}

impl<C: ChannelCount> Drop for SimpleClient<C> {
    fn drop(&mut self) {
        unsafe { pa_simple_free(self.simple) };
    }
}


pub struct Playback<C: ChannelCount> {
    client: SimpleClient<C>
}

impl<C: ChannelCount> Playback<C> {
    pub fn new(name: &str, desc: &str, rate: u32) -> Self {
        Playback {
            client: SimpleClient::new(name, desc, PA_STREAM_PLAYBACK, rate)
        }
    }

    pub fn write(&self, data: &[C]) {
        let res = unsafe {
            let ptr = transmute(data.as_ptr());
            pa_simple_write(self.client.simple, ptr, data.len() * C::sample_size(), null_mut())
        };
        assert!(res == 0);
    }
}

#[test]
fn test_playback() {
    let p = Playback::new("Test", "Playback", 48000);

    // Generate sound
    let mut data = Vec::with_capacity(4800);
    for _ in 0..4800 {
        data.push([0]);
    }

    // Play
    p.write(&data[..]);
}


pub struct Record<C: ChannelCount> {
    client: SimpleClient<C>
}

impl<C: ChannelCount> Record<C> {
    pub fn new(name: &str, desc: &str, rate: u32) -> Self {
        Record {
            client: SimpleClient::new(name, desc, PA_STREAM_RECORD, rate)
        }
    }

    pub fn read(&self, data: &mut [C]) {
        let res = unsafe {
            let ptr = transmute(data.as_mut_ptr());
            pa_simple_read(self.client.simple, ptr, data.len() * C::sample_size(), null_mut())
        };
        assert!(res >= 0);
    }
}

#[test]
fn test_record() {
    let p = Record::new("Test", "Record", 48000);

    // Fill:
    let mut data = Vec::with_capacity(4800);
    for _ in 0..4800 {
        data.push([0, 0]);
    }

    // Record
    p.read(&mut data[..]);
}
