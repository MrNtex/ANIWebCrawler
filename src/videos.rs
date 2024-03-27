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
    title: String,
}
#[derive(Clone)]
pub struct VideoData {
    pub id: String,
    pub view_count: u64,
    pub published_at: String,
    pub title: String,
}

type video_data_list = (VideoData, VideoData);

pub async fn fetch_video_data(identifier: &str, api_key: &str) ->  Result<video_data_list, Box<dyn std::error::Error>> {
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

    // Initialize variables to store information about the most viewed and latest videos
    let mut most_viewed: Option<VideoData> = None;
    let mut latest: Option<VideoData> = None;

    for item in video_list_res.items {
        let view_count_curr = item.statistics.viewCount.parse::<u64>().unwrap_or(0);
        let video_detail = VideoData {
            id: item.id.clone(),
            title: item.snippet.title.clone(),
            published_at: item.snippet.publishedAt.clone(),
            view_count: view_count_curr,
        };

        // Determine if current video is the most viewed
        if most_viewed.as_ref().map_or(true, |v: &VideoData| view_count_curr > v.view_count) {
            most_viewed = Some(video_detail.clone());
        }

        // Determine if current video is the latest published
        if latest.as_ref().map_or(true, |v: &VideoData| item.snippet.publishedAt > v.published_at) {
            latest = Some(video_detail);
        }
    }

    if let (Some(latest_video), Some(most_viewed_video)) = (latest, most_viewed) {
        Ok((latest_video, most_viewed_video.clone()))
    } else {
        Err("No videos found".into())
    }
}