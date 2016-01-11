This module is uses the *libpulse-sys* bindings to implement a Rust
interface to the
[pulse-simple API](http://freedesktop.org/software/pulseaudio/doxygen/simple_8h.html).
It features guessing sample type and channel count based on your use
of `Playback.write()` and `Record.read()`. See the examples for some
basic usage.
