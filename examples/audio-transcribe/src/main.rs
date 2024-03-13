use async_openai::{
    types::{AudioResponseFormat, CreateTranscriptionRequestArgs, TimestampGranularity},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // Credits and Source for audio: https://www.youtube.com/watch?v=oQnDVqGIv4s
    let request = CreateTranscriptionRequestArgs::default()
        .file(
            "./audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3",
        )
        .model("whisper-1")
        .build()?;

    let response = client.audio().transcribe(request).await?;

    println!("{}", response.text);

    // Verbose JSON with segments and words
    let request = CreateTranscriptionRequestArgs::default()
        .file(
            "./audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3",
        )
        .model("whisper-1")
        .response_format(AudioResponseFormat::VerboseJson)
        .timestamp_granularities(vec![
            TimestampGranularity::Segment,
            TimestampGranularity::Word,
        ])
        .build()?;

    let response = client.audio().transcribe(request).await?;

    println!("{:?}", response.segments.unwrap_or_default());
    println!("{:?}", response.words.unwrap_or_default());

    Ok(())
}
