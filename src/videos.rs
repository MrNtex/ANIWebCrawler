use serde::Deserialize;

#[derive(Deserialize)]
struct SearchResponse {
    items: Vec<SearchItem>,
}

#[derive(Deserialize)]
struct SearchItem {
    id: SearchItemId,
}

#[derive(Deserialize)]
struct SearchItemId {
    videoId: Option<String>,
}

#[derive(Deserialize)]
struct VideoListResponse {
    items: Vec<VideoItem>,
}

#[derive(Deserialize)]
struct VideoItem {
    id: String,
    statistics: VideoStatistics,
    snippet: VideoSnippet,
}

#[derive(Deserialize)]
struct VideoStatistics {
    viewCount: String,
}

#[derive(Deserialize)]
struct VideoSnippet {
    publishedAt: String,
}

pub async fn fetch_video_data(identifier: &str, api_key: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let search_url = format!(
        "https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults=20",
        api_key, identifier
    );

    let client = reqwest::Client::new();
    let search_res = client.get(&search_url).send().await?.json::<SearchResponse>().await?;

    let video_ids: Vec<String> = search_res.items.into_iter()
        .filter_map(|item| item.id.videoId)
        .collect();

    let video_details_url = format!(
        "https://www.googleapis.com/youtube/v3/videos?key={}&id={}&part=statistics,snippet",
        api_key,
        video_ids.join(",")
    );

    let video_list_res = client.get(&video_details_url).send().await?.json::<VideoListResponse>().await?;

    let (most_viewed, latest) = video_list_res.items.into_iter()
        .fold((None, None), |(max_view, latest), item| {
            let view_count = item.statistics.viewCount.parse::<u64>().unwrap_or(0);
            let max_view = match max_view {
                Some((id, max_views)) if view_count > max_views => Some((item.id.clone(), view_count)),
                None => Some((item.id.clone(), view_count)),
                _ => max_view,
            };
            let latest = match latest {
                Some((id, date)) if item.snippet.publishedAt > date => Some((item.id, item.snippet.publishedAt)),
                None => Some((item.id, item.snippet.publishedAt)),
                _ => latest,
            };
            (max_view, latest)
        });

    // Extracting the video IDs from the tuples
    let most_viewed_video_id = most_viewed.map(|(id, _)| id).unwrap_or_default();
    let latest_video_id = latest.map(|(id, _)| id).unwrap_or_default();

    Ok((most_viewed_video_id, latest_video_id))
}