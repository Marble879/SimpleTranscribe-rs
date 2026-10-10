# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`simple_transcribe_rs` is a small Rust library crate (published to crates.io) that wraps [whisper-rs](https://github.com/tazz4843/whisper-rs) to make audio-to-text transcription simple: it downloads the Whisper GGML model on demand, decodes the audio file, and runs inference.

## Commands

```sh
cargo build
cargo test                                   # what CI runs on PRs to master (build + test)
cargo test transcriber::tests::component_test_happy_case   # single test
cargo test -- --test-threads=1              # see "Tests" below for why this can matter
cargo run --example usage_example           # tiny model, English (src/test_data/test.mp3)
cargo run --example usage_example_chinese   # large model (~3 GB download), Chinese
```

Building requires `cmake` and `clang`/libclang, since `whisper-rs` compiles whisper.cpp from source and generates bindings.

## Architecture

The public API is two modules; `audio_parser` is private.

- **`model_handler::ModelHandler::new(name, dir)`** (async): maps a user-facing name (`tiny`/`base`/`small`/`medium`/`large`, case-insensitive) to a GGML filename via the `phf` `MODEL_MAP`. If `{dir}/{ggml-name}.bin` is missing, it downloads it from `huggingface.co/ggerganov/whisper.cpp`. This async step is the reason consumers need a Tokio runtime. Download errors are silently discarded (`let _ =`). An unknown model name panics.
- **`transcriber::Transcriber::new(ModelHandler)`** loads a `WhisperContext` from the model path. **`transcribe(path, Option<FullParams>)`** is synchronous. It decodes the audio, creates a fresh `WhisperState` for each call, runs `full()`, and joins all segment texts into one `TranscriberOutput`. If no params are given, it defaults to greedy sampling with `best_of: 1`. Callers can pass their own `whisper_rs::FullParams`, e.g. to set the language (see the Chinese example).
- **`audio_parser::parse_audio_file`** uses symphonia to probe and decode the file into interleaved `f32` samples. It does **no resampling or downmixing**, even though Whisper expects 16 kHz mono. `test.mp3` is 24 kHz mono, and the exact expected string in the transcriber test depends on this behavior. If you add resampling, that test's expected text will change.

Things to know about the current behavior:
- Most failures panic (`unwrap`/`expect`) instead of returning `Err`, even though `transcribe` returns a `Result`.
- `TranscriberOutput`'s start and end timestamps are overwritten on each loop iteration, so they hold only the **last** segment's `t0`/`t1` (Whisper units: centiseconds).

## Tests

- Tests need network access. They download the real `ggml-tiny` model from Hugging Face.
- Tests use fixed relative directories under the crate root (`models/`, `test_models/`, `test_dir/`) and delete them with `remove_dir_all`. The two `model_handler` tests share `test_models/`, so with parallel test threads they can race. The transcriber test deletes `models/`, which is the same directory the examples download into.
- `test_check_model_exists_existent_path` creates a *directory* named `ggml-tiny.bin` so that the existence check passes without downloading.

## Release process

The version in `Cargo.toml` stays at the placeholder `0.0.0`; do not bump it by hand. Pushing a `v*` tag triggers `.github/workflows/deployment.yml`. That workflow uses `sed` to replace `0.0.0` with the tag's version and runs `cargo publish`. Dependabot opens PRs to keep dependencies up to date (most recent commits are these bumps).
