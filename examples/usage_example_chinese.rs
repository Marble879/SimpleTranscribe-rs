use simple_transcribe_rs::model_handler;
use simple_transcribe_rs::transcriber;
use whisper_rs::FullParams;
use whisper_rs::SamplingStrategy;

#[tokio::main]
async fn main() {
    let m = model_handler::ModelHandler::new("large", "models/").await;
    let trans = transcriber::Transcriber::new(m);
    let mut params = FullParams::new(SamplingStrategy::default());
    params.set_language(Some("zh"));

    let result = trans.transcribe("src/test_data/gongxifachai.mp3", Some(params)).unwrap();
    // 16KHz Sample rate, Mono for language other than English is better for Whisper.
    let text = result.get_text();
    let start = result.get_start_timestamp();
    let end = result.get_end_timestamp();
    println!("start[{}]-end[{}] {}", start, end, text);
}
