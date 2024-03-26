use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    items: Vec<Channel>,
}

#[derive(Deserialize, Debug)]
struct Channel {
    statistics: Statistics,
}

#[derive(Deserialize, Debug)]
struct Statistics {
    #[serde(rename = "viewCount")]
    view_count: Option<String>,
    
    #[serde(rename = "subscriberCount")]
    subscriber_count: Option<String>,
    
    #[serde(rename = "videoCount")]
    video_count: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <channel_id> <api_key>", args[0]);
        std::process::exit(1);
    }

    // Adjusted indices for channel_id and api_key
    fetch_channel_data(&args[1], &args[2]).await?;

    Ok(())
}

async fn fetch_channel_data(channel_id: &str, api_key: &str) -> Result<(), Error> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/channels?part=statistics&id={}&key={}",
        channel_id, api_key
    );

    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?.json::<ApiResponse>().await?;

    if let Some(channel) = res.items.first() {
        let view_count = channel.statistics.view_count.as_deref().unwrap_or("N/A");
        let subscriber_count = channel.statistics.subscriber_count.as_deref().unwrap_or("N/A");
        let video_count = channel.statistics.video_count.as_deref().unwrap_or("N/A");

        println!("View count: {}", view_count);
        println!("Subscriber count: {}", subscriber_count);
        println!("Video count: {}", video_count);
    } else {
        eprintln!("No channel found");
    }

    Ok(())
}
