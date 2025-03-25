# RSSTV
RSSTV is a SSTV transcoder written in rust, supporting encoding, decoding
live streaming from the microphone.

## Current features
- Martin M1 transcoding
- Microphone streaming
- A base to work off to add more modes

## Planned features
- More modes
- Faster decoding (currently takes 100ms per partial decode in a live decode)
- A website powered by WASM