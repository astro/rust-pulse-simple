extern crate pulse_simple;

use pulse_simple::Playback;
use std::f64::consts::PI;


const RATE: u32 = 48000;

fn main() {
    let p = Playback::new("Example", "Playback", 2, RATE);

    // Generate 1s of sound
    let mut data = Vec::with_capacity(RATE as usize);
    for i in 0..RATE {
        let t = i as f64 / RATE as f64;
        let mut push_freq = |f: f64| data.push(((std::i16::MAX as f64) * (f * 2.0 * PI * t).sin()) as i16);
        push_freq(440.0);
        push_freq(330.0);
    }

    // Play in a loop
    loop {
        p.write(&data[..]);
    }
}
