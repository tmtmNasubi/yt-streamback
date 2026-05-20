use std::str;

use reqwest::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YouTubeSearchListResponse {
    pub kind: String,
    pub etag: String,

    #[serde(default)]
    pub next_page_token: Option<String>,

    #[serde(default)]
    pub prev_page_token: Option<String>,

    #[serde(default)]
    pub region_code: Option<String>,

    pub page_info: PageInfo,
    pub items: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total_results: u32,
    pub results_per_page: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub kind: String,
    pub etag: String,
    pub id: SearchResultId,
    pub snippet: SearchResultSnippet,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultId {
    pub kind: String,

    #[serde(default)]
    pub video_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultSnippet {
    pub published_at: DateTime<Utc>,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    pub channel_title: String,
    pub live_broadcast_content: String,
    pub publish_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    #[serde(default)]
    pub default: Option<Thumbnail>,

    #[serde(default)]
    pub medium: Option<Thumbnail>,

    #[serde(default)]
    pub high: Option<Thumbnail>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,

    #[serde(default)]
    pub width: Option<u32>,

    #[serde(default)]
    pub height: Option<u32>,
}

pub async fn search_movies(query: &str, key: &str) -> Result<Vec<SearchResult>, reqwest::Error>  {
    let client: Client = reqwest::Client::new();
    let res: YouTubeSearchListResponse = client
        .get("https://www.googleapis.com/youtube/v3/search")
        .query(&[
            ("part", "snippet"),
            ("type", "video"),
            ("q", query),
            ("maxResults", "50"),
            ("key", key),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<YouTubeSearchListResponse>()
        .await?;

    Ok(res.items)
}
