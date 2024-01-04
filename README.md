# SimpleTranscribe-rs 🔈 📖
An audio to text transcription library written in rust that utilizes [Whisper-rs](https://github.com/tazz4843/whisper-rs) bindings.

## What is SimpleTranscribe-rs?
SimpleTranscribe-rs is a library written in Rust with the goal of making audio to text transcription simple for developers. SimpleTranscribe-rs handles different aspects of transcription such as automatically downloading required whisper text to speech models. The aim is for developers to be able to incorporate transcription in their projects quickly 🌩️

## Features
- Automatically downloads Models that have no already been installed. Supported models:
    - Tiny
    - Base
    - Small
    - Medium
    - Large

- Transcribes audio from different file types such as:
  - mp3
  - wav 


## Getting started
To use SimpleTranscribe-rs, simply add it to your project's `cargo.toml`:
```
[dependencies]
simple_transcribe_rs = "1.0.1"
tokio = { version = "1.35.1", features = ["full"] }
```

Due to the nature of downloading models, it is necessary to await instantiations of the model handler. Therefore an async runtime is required.
[Tokio](https://github.com/tokio-rs/tokio) is what is used internally in the library and has also been tested with, and therefore is the recommended runtime for this library.

## Usage
To use SimpleTranscribe-rs, the model handler first needs to be used to setup and prepare the language model. Afterwards, the transcriber can be used to 
convert audio files to text. The following snippet depicts an example of this:

```rust
use simple_transcribe_rs::model_handler;
use simple_transcribe_rs::transcriber;

#[tokio::main]
async fn main() {
    let m = model_handler::ModelHandler::new("tiny", "models/").await;
    let trans = transcriber::Transcriber::new(m);
    let result = trans.transcribe("src/test_data/test.mp3", None).unwrap();
    let text = result.get_text();
    let start = result.get_start_timestamp();
    let end = result.get_end_timestamp();
    println!("start[{}]-end[{}] {}", start, end, text);
}
```

The snippet can be run via:
```cargo run --example usage_example```
