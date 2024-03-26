use reqwest::Error;
use serde::Deserialize;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let data = read_lines("channels.txt").unwrap();
    // if args.len() < 4 {
    //     eprintln!("Usage: {} <identifier> <api_key> <useId>", args[0]);
    //     std::process::exit(1);
    // }
    let use_id = data[2].eq("true");
    // Adjusted indices for channel_id and api_key
    fetch_channel_data(&data[0], &data[1], use_id).await?;

    Ok(())
}

async fn fetch_channel_data(identifier: &str, api_key: &str, use_id: bool) -> Result<(), Error> {
    
    let (query_param, identifier_value) = if !use_id {
        ("forUsername", identifier)
    } else {
        ("id", identifier)
    };
    
    let url = format!(
        "https://www.googleapis.com/youtube/v3/channels?part=statistics&{}={}&key={}",
        query_param, identifier_value, api_key
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
