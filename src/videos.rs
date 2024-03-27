use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    items: Vec<SearchResult>,
}

#[derive(Deserialize, Debug)]
struct SearchResult {
    id: VideoId,
    snippet: Snippet,
}

#[derive(Deserialize, Debug)]
struct VideoId {
    videoId: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Snippet {
    // Add fields from the snippet you're interested in, for example:
    title: String,
    description: String,
    // Add more fields as needed
}

pub async fn fetch_video_data(identifier: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults=20",
        api_key, identifier
    );

    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?.json::<ApiResponse>().await?;

    let mut video_ids = Vec::new();
    for item in res.items {
        if let Some(video_id) = item.id.videoId {
            println!("Video ID: {}", video_id); // Displaying video ID, you could also print title or description
            video_ids.push(video_id);
        }
    }
    Ok(())
}