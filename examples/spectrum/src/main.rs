extern crate pulse_simple;
extern crate rustfft;
extern crate num;

use pulse_simple::Record;
use rustfft::FFT;
use num::complex::Complex;


fn analyze_channel(fft: &mut FFT<f32>, data: &[[f32; 2]], channel: usize) -> Vec<f32> {
    let mut input = Vec::with_capacity(data.len());
    for x in data {
        input.push(Complex::new(x[channel], 0.0));
    }
    
    let mut output = input.clone();
    fft.process(&input[..], &mut output);
    
    let mut result = Vec::with_capacity(data.len());
    for ref c in output[(output.len() / 2)..output.len()].iter() {
        result.push(c.norm());
    }
    result
}

const RATE: u32 = 48000;
const WINDOW: usize = 4800;
const FREQS_PER_COLUMN: usize = 40;

fn main() {
    let p = Record::new("Example", "Record", RATE);
    let mut fft = FFT::new(WINDOW, false);

    // Fill:
    let mut data = Vec::with_capacity(WINDOW);
    for _ in 0..WINDOW {
        data.push([0.0, 0.0]);
    }

    // Record:
    let mut max: f32 = 0.0;
    loop {
        p.read(&mut data[..]);
        let freqs = analyze_channel(&mut fft, &data[..], 0);

        let mut top_freq = 0.0;
        let mut top_freq_volume = 0.0;
        for (i, volume) in freqs.iter().enumerate() {
            if volume >= &top_freq_volume {
                top_freq = i as f32 * RATE as f32 / freqs.len() as f32;
                top_freq_volume = *volume;
            }
        }
        println!("top: {} at {}", top_freq, top_freq_volume);
        
        let mut spectrum = Vec::with_capacity(WINDOW / FREQS_PER_COLUMN);
        max *= 0.95;  // Dampen
        for column in 0..(WINDOW / 2 / FREQS_PER_COLUMN) {
            let c1 = column * FREQS_PER_COLUMN;
            let c2 = (column + 1) * FREQS_PER_COLUMN;
            let mut sum = 0.0;
            for x in c1..c2 {
                sum += freqs[x];
            }
            spectrum.push(sum);
            max = max.max(sum);
        }

        print!("[");
        for s in spectrum {
            print!("{}", (9.0 * s.max(0.0) / max) as u8);
        }
        println!("]");
    }
}
