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

struct SimpleClient<S: Sampleable> {
    simple: *mut pa_simple,
    phantom: PhantomData<S>
}

impl<S: Sampleable> SimpleClient<S> {
    fn new(name: &str, desc: &str, dir: pa_stream_direction_t, channel_count: u8, rate: u32) -> Self {
        let ss = pa_sample_spec {
            format: S::format(),
            channels: channel_count,
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

impl<S: Sampleable> Drop for SimpleClient<S> {
    fn drop(&mut self) {
        unsafe { pa_simple_free(self.simple) };
    }
}


pub struct Playback<S: Sampleable> {
    client: SimpleClient<S>
}

impl<S: Sampleable> Playback<S> {
    pub fn new(name: &str, desc: &str, channel_count: u8, rate: u32) -> Self {
        Playback {
            client: SimpleClient::new(name, desc, PA_STREAM_PLAYBACK, channel_count, rate)
        }
    }

    pub fn write(&self, data: &[S]) {
        let res = unsafe {
            let ptr = transmute(data.as_ptr());
            pa_simple_write(self.client.simple, ptr, data.len() * size_of::<S>(), null_mut())
        };
        assert!(res == 0);
    }
}

#[test]
fn test_playback() {
    let p = Playback::new("Test", "Playback", 1, 48000);

    // Generate sound
    let mut data = Vec::with_capacity(4800);
    for _ in 0..4800 {
        data.push(0);
    }

    // Play
    p.write(&data[..]);
}


pub struct Record<S: Sampleable> {
    client: SimpleClient<S>
}

impl<S: Sampleable> Record<S> {
    pub fn new(name: &str, desc: &str, channel_count: u8, rate: u32) -> Self {
        Record {
            client: SimpleClient::new(name, desc, PA_STREAM_RECORD, channel_count, rate)
        }
    }

    pub fn read(&self, data: &mut [S]) {
        let res = unsafe {
            let ptr = transmute(data.as_mut_ptr());
            pa_simple_read(self.client.simple, ptr, data.len() * size_of::<S>(), null_mut())
        };
        assert!(res >= 0);
    }
}

#[test]
fn test_record() {
    let p = Record::new("Test", "Record", 1, 48000);

    // Fill:
    let mut data = Vec::with_capacity(4800);
    for _ in 0..4800 {
        data.push(0);
    }

    // Play
    p.read(&mut data[..]);
}
