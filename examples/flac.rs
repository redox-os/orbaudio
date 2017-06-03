extern crate claxon;
extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct Flac<R: claxon::input::ReadBytes + Send> {
    samples: claxon::FlacSamples<R>
}

impl<R: claxon::input::ReadBytes + Send> AudioCallback for Flac<R> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            if let Some(sample) = self.samples.next() {
                *x = (sample.unwrap() as f32) * 0.25;
            }
        }
    }
}

fn main() {
    let mut reader = claxon::FlacReader::open("res/elements.flac").unwrap();
    let info = reader.streaminfo();

    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(info.sample_rate as i32),
        channels: Some(info.channels as u8),
        samples: None
    };

    println!("{:?}", info);
    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);
        Flac {
            samples: reader.samples()
        }
    }).unwrap();

    // Start playback
    device.resume();

    // Play for 2 seconds
    std::thread::sleep(Duration::from_millis(5000));
}
