extern crate cpal;

use cpal::{
  EventLoop, Format, SampleFormat, SampleRate, StreamData,
  UnknownTypeOutputBuffer
};

fn main() {
  let event_loop = EventLoop::new();

  let device = cpal::default_output_device().unwrap();

  let format = Format {
    channels: 2,
    data_type: SampleFormat::F32,
    sample_rate: SampleRate(48_000),
  };

  let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

  event_loop.play_stream(stream_id);

  event_loop.run(move |_stream_id, stream_data| {
    if let StreamData::Output { buffer } = stream_data {
      if let UnknownTypeOutputBuffer::F32(mut buffer) = buffer {
        println!("{}", buffer.len());
        for sample in buffer.iter_mut() {
          *sample = 0.0;
        }
      } else {
        panic!("expected f32 audio stream");
      }
    } else {
      panic!("expected audio output stream");
    }
  });
}
