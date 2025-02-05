use std::{
    thread,
    time::Duration,
};

use rtaudio::{
    Api,
    Buffers,
    DeviceParams,
    SampleFormat,
    StreamInfo,
    StreamOptions,
    StreamStatus,
};

const AMPLITUDE: f32 = 0.5;
// const FREQ_HZ: f32 = 440.0;

fn main() {
    let a = thread::spawn(|| test(0, 440.));
    let b = thread::spawn(|| test(1, 880.));

    a.join().unwrap();
    b.join().unwrap();
}

pub fn test(channel: u32, freq: f32) {
    let host = rtaudio::Host::new(Api::Unspecified).unwrap();

    dbg!(host.api());

    let out_device = host.default_output_device().unwrap();

    let mut stream_handle = host
        .open_stream(
            Some(DeviceParams {
                device_id: out_device.id,
                num_channels: 1,
                first_channel: channel,
            }),
            None,
            SampleFormat::Float32,
            out_device.preferred_sample_rate,
            64,
            StreamOptions::default(),
            |error| eprintln!("{}", error),
        )
        .unwrap();

    dbg!(stream_handle.info());

    let mut phasor = 0.0;
    let phasor_inc = freq / stream_handle.info().sample_rate as f32;

    stream_handle
        .start(
            move |buffers: Buffers<'_>, _info: &StreamInfo, _status: StreamStatus| {
                if let Buffers::Float32 { output, input: _ } = buffers {
                    // By default, buffers are interleaved.
                    for frame in output {
                        // .chunks_mut(2) {
                        // Generate a sine wave at 440 Hz at 50% volume.
                        let val = (phasor * std::f32::consts::TAU).sin() * AMPLITUDE;
                        phasor = (phasor + phasor_inc).fract();

                        *frame = val;

                        // frame[0] = val;
                        // frame[1] = val;
                    }
                }
            },
        )
        .unwrap();

    // Wait 2 seconds before closing.
    thread::sleep(Duration::from_secs(2));
}
