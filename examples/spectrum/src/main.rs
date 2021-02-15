extern crate pulse_simple;
extern crate dft;

use pulse_simple::Record;
use dft::{Operation, Plan};


fn analyze_channel(plan: &Plan<f64>, data: &[[f32; 2]], channel: usize) -> Vec<f32> {
    let mut input = Vec::with_capacity(data.len());
    for x in data {
        input.push(x[channel] as f64);
    }
    
    dft::transform(&mut input, &plan);
    let output = dft::unpack(&input);
    
    let mut result = Vec::with_capacity(data.len());
    for ref c in output {
        result.push(c.norm() as f32);
    }
    result
}

const RATE: u32 = 48000;
const WINDOW: usize = 2048;
const FREQS_PER_COLUMN: usize = 20;

fn main() {
    let p = Record::new("Example", "Record", None, RATE);
    let mut plan = Plan::new(Operation::Forward, WINDOW);

    // Fill:
    let mut data = Vec::with_capacity(WINDOW);
    for _ in 0..WINDOW {
        data.push([0.0, 0.0]);
    }

    // Record:
    let mut max: f32 = 0.0;
    loop {
        p.read(&mut data[..]);
        let freqs = analyze_channel(&mut plan, &data[..], 0);

        let mut top_freq = 0.0;
        let mut top_freq_volume = 0.0;
        for (i, volume) in freqs.iter().enumerate() {
            if i > 0 && i < freqs.len() / 2 && volume >= &top_freq_volume {
                top_freq = i as f32 * RATE as f32 / freqs.len() as f32;
                top_freq_volume = *volume;
            }
        }
        println!("top: {} Hz at volume {}", top_freq, top_freq_volume);
        
        let mut spectrum = Vec::with_capacity(WINDOW / FREQS_PER_COLUMN);
        max *= 0.95;  // Dampen
        for column in 0..(WINDOW / FREQS_PER_COLUMN) {
            let c1 = column * FREQS_PER_COLUMN;
            let c2 = (column + 1) * FREQS_PER_COLUMN;
            let mut sum: f32 = 0.0;
            for x in c1..c2 {
                sum += freqs[x];
            }
            if column > 0 && column < WINDOW / FREQS_PER_COLUMN / 2 {
                spectrum.push(sum);
                max = max.max(sum);
            }
        }

        print!("[");
        for s in spectrum {
            print!("{}", (9.0 * s.max(0.0) / max) as u8);
        }
        println!("]");
    }
}
