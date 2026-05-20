use crate::clients::youtube::search_movies;
use crate::clients::youtube::SearchResult;

pub async fn find_backlink(query: &str, api_key: &str) -> Result<Vec<SearchResult>, reqwest::Error> {
    let video_id = query;
    let items = search_movies(video_id, api_key).await?;

    let filtered = items
        .into_iter()
        .filter(|item| {
            item.id
                .video_id
                .as_deref()
                .is_some_and(|id| id != video_id)
        })
        .collect();

    Ok(filtered)
}
