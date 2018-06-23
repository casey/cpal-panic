# cpal-panic

A minimal example demonstrating [cpal/issues/228](https://github.com/tomaka/cpal/issues/228).

Running it produces the following backtrace on macOS 10.13.4:

```
RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/cpal-panic`
thread '<unnamed>' panicked at 'It is forbidden to read from the audio buffer', /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/cpal-0.8.1/src/lib.rs:603:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:207
   3: std::panicking::default_hook
             at libstd/panicking.rs:223
   4: std::panicking::begin_panic
             at libstd/panicking.rs:402
   5: std::panicking::begin_panic
             at /Users/travis/build/rust-lang/rust/src/libstd/panicking.rs:365
   6: <cpal::OutputBuffer<'a, T> as core::ops::deref::Deref>::deref
             at ./<panic macros>:3
   7: cpal_panic::main::{{closure}}
             at src/main.rs:26
   8: cpal::EventLoop::run::{{closure}}
             at /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/cpal-0.8.1/src/lib.rs:489
   9: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &'a mut F>::call_mut
             at /Users/travis/build/rust-lang/rust/src/libcore/ops/function.rs:261
  10: cpal::cpal_impl::EventLoop::build_output_stream::{{closure}}
             at /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/cpal-0.8.1/src/coreaudio/mod.rs:744
  11: coreaudio::audio_unit::render_callback::<impl coreaudio::audio_unit::AudioUnit>::set_render_callback::{{closure}}
             at /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/coreaudio-rs-0.9.1/src/audio_unit/render_callback.rs:432
  12: <core::slice::Iter<'a, T> as core::iter::iterator::Iterator>::next
             at /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/coreaudio-rs-0.9.1/src/audio_unit/render_callback.rs:673
  13: _ZN14AUInputElement9PullInputERjRK14AudioTimeStampjj
  14: _ZN23AUInputFormatConverter29InputProcEP20OpaqueAudioConverterPjP15AudioBufferListPP28AudioStreamPacketDescriptionPv
  15: _ZN19AudioConverterChain13CallInputProcEj
  16: _ZN19AudioConverterChain23FillBufferFromInputProcEPjP12CABufferList
  17: _ZN22BufferedAudioConverter13GetInputBytesEjRjRPK12CABufferList
  18: _ZN17Resampler2Wrapper12RenderOutputEP12CABufferListjRj
  19: _ZN19SampleRateConverter12RenderOutputEP12CABufferListjRjP28AudioStreamPacketDescription
  20: _ZN22BufferedAudioConverter10FillBufferERjR15AudioBufferListP28AudioStreamPacketDescription
  21: _ZN19AudioConverterChain12RenderOutputEP12CABufferListjRjP28AudioStreamPacketDescription
  22: _ZN22BufferedAudioConverter10FillBufferERjR15AudioBufferListP28AudioStreamPacketDescription
  23: AudioConverterFillComplexBuffer
  24: _ZN23AUInputFormatConverter219PullAndConvertInputERK14AudioTimeStampRjR15AudioBufferListP28AudioStreamPacketDescriptionRb
  25: _ZN15AUConverterBase9RenderBusERjRK14AudioTimeStampjj
  26: _ZN6AUBase11DoRenderBusERjRK14AudioTimeStampjP15AUOutputElementjR15AudioBufferList
  27: _ZN6AUBase8DoRenderERjRK14AudioTimeStampjjR15AudioBufferList
  28: _ZN5AUHAL8AUIOProcEjPK14AudioTimeStampPK15AudioBufferListS2_PS3_S2_Pv
  29: _ZN19HALC_ProxyIOContext10IOWorkLoopEv
  30: _ZN19HALC_ProxyIOContext13IOThreadEntryEPv
  31: _ZN13HALB_IOThread5EntryEPv
  32: _pthread_body
  33: _pthread_start
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "PoisonError { inner: .. }"', libcore/result.rs:945:5
stack backtrace:
   0:        0x109b3e1cb - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h4de839a9fdceed9d
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:        0x109b323fb - std::sys_common::backtrace::print::hc90998cb84923459
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:        0x109b3a11d - std::panicking::default_hook::{{closure}}::ha08d7ed892ce8830
                               at libstd/panicking.rs:207
   3:        0x109b39e7a - std::panicking::default_hook::hba629fe4bbe02041
                               at libstd/panicking.rs:223
   4:        0x109b3a536 - std::panicking::begin_panic::hfff0559644f54b1a
                               at libstd/panicking.rs:402
   5:        0x109b3a36a - std::panicking::try::do_call::hd1b51a136314709e
                               at libstd/panicking.rs:349
   6:        0x109b3a262 - std::panicking::try::do_call::hd1b51a136314709e
                               at libstd/panicking.rs:325
   7:        0x109b74705 - core::ptr::drop_in_place::h446bbb18662fbc13
                               at libcore/panicking.rs:72
   8:        0x109b1ba98 - core::result::unwrap_failed::h6f088030986b356d
                               at /Users/travis/build/rust-lang/rust/src/libcore/macros.rs:26
   9:        0x109b1b79f - <core::result::Result<T, E>>::unwrap::hef56ce5b845c9843
                               at /Users/travis/build/rust-lang/rust/src/libcore/result.rs:782
  10:        0x109b219ba - cpal::cpal_impl::EventLoop::build_output_stream::{{closure}}::h79b4ae6d2144e89e
                               at /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/cpal-0.8.1/src/coreaudio/mod.rs:725
  11:        0x109b249c6 - coreaudio::audio_unit::render_callback::<impl coreaudio::audio_unit::AudioUnit>::set_render_callback::{{closure}}::h1bf1346dcd4842a4
                               at /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/coreaudio-rs-0.9.1/src/audio_unit/render_callback.rs:432
  12:        0x109b2ee00 - <core::slice::Iter<'a, T> as core::iter::iterator::Iterator>::next::h1999a926a097bbe5
                               at /Users/rodarmor/.cargo/registry/src/github.com-1ecc6299db9ec823/coreaudio-rs-0.9.1/src/audio_unit/render_callback.rs:673
  13:        0x10ccaeda8 - _ZN14AUInputElement9PullInputERjRK14AudioTimeStampjj
  14:        0x10cb8bf96 - _ZN23AUInputFormatConverter29InputProcEP20OpaqueAudioConverterPjP15AudioBufferListPP28AudioStreamPacketDescriptionPv
  15:     0x7fff4a374256 - _ZN19AudioConverterChain13CallInputProcEj
  16:     0x7fff4a373f0c - _ZN19AudioConverterChain23FillBufferFromInputProcEPjP12CABufferList
  17:     0x7fff4a373e6e - _ZN22BufferedAudioConverter13GetInputBytesEjRjRPK12CABufferList
  18:     0x7fff4a3fe187 - _ZN17Resampler2Wrapper12RenderOutputEP12CABufferListjRj
  19:     0x7fff4a38420e - _ZN19SampleRateConverter12RenderOutputEP12CABufferListjRjP28AudioStreamPacketDescription
  20:     0x7fff4a366989 - _ZN22BufferedAudioConverter10FillBufferERjR15AudioBufferListP28AudioStreamPacketDescription
  21:     0x7fff4a366cd9 - _ZN19AudioConverterChain12RenderOutputEP12CABufferListjRjP28AudioStreamPacketDescription
  22:     0x7fff4a366989 - _ZN22BufferedAudioConverter10FillBufferERjR15AudioBufferListP28AudioStreamPacketDescription
  23:     0x7fff4a36631e - AudioConverterFillComplexBuffer
  24:        0x10cb8b6ca - _ZN23AUInputFormatConverter219PullAndConvertInputERK14AudioTimeStampRjR15AudioBufferListP28AudioStreamPacketDescriptionRb
  25:        0x10cb8b50b - _ZN15AUConverterBase9RenderBusERjRK14AudioTimeStampjj
  26:        0x10ccb5ca3 - _ZN6AUBase11DoRenderBusERjRK14AudioTimeStampjP15AUOutputElementjR15AudioBufferList
  27:        0x10ccb5379 - _ZN6AUBase8DoRenderERjRK14AudioTimeStampjjR15AudioBufferList
  28:        0x10cb8ed43 - _ZN5AUHAL8AUIOProcEjPK14AudioTimeStampPK15AudioBufferListS2_PS3_S2_Pv
  29:     0x7fff4b42a884 - _ZN19HALC_ProxyIOContext10IOWorkLoopEv
  30:     0x7fff4b42931b - _ZN19HALC_ProxyIOContext13IOThreadEntryEPv
  31:     0x7fff4b42905d - _ZN13HALB_IOThread5EntryEPv
  32:     0x7fff74007660 - _pthread_body
  33:     0x7fff7400750c - _pthread_start
thread panicked while panicking. aborting.
error: Recipe `default` was terminated on line 2 by signal 4
```
