use simple_transcribe_rs::model_handler;
use simple_transcribe_rs::transcriber;

#[tokio::main]
async fn main() {
    let m = model_handler::ModelHandler::new("tiny", "models/").await;
    let trans = transcriber::Transcriber::new(m);
    let result = trans.transcribe("src/test.mp3", None).unwrap();
    let text = result.get_text();
    println!("{}", text);
}
