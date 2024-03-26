use reqwest::Error;
use serde::Deserialize;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use chrono::prelude::*;
use colored::Colorize;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = read_lines("channels.txt").unwrap();
    let mut save_to_txt = false;
    if args.len() < 2 {
        println!("{} - Please provide a boolean value to save to txt file (assumed false)", "warn".yellow());
    }else if args[1].eq("true"){
        save_to_txt = true;
    }
    let use_id = data[2].eq("true");
    // Adjusted indices for channel_id and api_key
    fetch_channel_data(&data[0], &data[1], use_id, save_to_txt).await?;

    Ok(())
}

async fn fetch_channel_data(identifier: &str, api_key: &str, use_id: bool, save_to_txt: bool) -> Result<(), Box<dyn std::error::Error>> {
    
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
        if save_to_txt {
            let dt = chrono::offset::Local::now();
            let date = dt.format("%Y-%m-%d %H:%M:%S").to_string();

            let existing_data = std::fs::read_to_string("channel_data.txt")?;

            let new_content = format!("\n[{date}]\nView count: {}\nSubscriber count: {}\nVideo count: {}\n\n{}", view_count, subscriber_count, video_count, existing_data);

            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("channel_data.txt")?;
            file.write_all(new_content.as_bytes())?;
        }
    } else {
        eprintln!("No channel found");
    }

    Ok(())
}
