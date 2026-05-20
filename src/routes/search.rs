use worker::*;
use crate::usecases::find_backlink::find_backlink;

pub async fn search(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let api_key = ctx.env.var("YOUTUBE_API_KEY")?.to_string();

    let Some(video_id) = url
        .query_pairs()
        .find(|(key, _)| key == "video_id")
        .map(|(_, value)| value.to_string())
    else {
        return Response::error("missing video_id", 400);
    };

    let items = find_backlink(&video_id, &api_key)
        .await
        .map_err(|e| Error::RustError(format!("failed to find backlinks: {e}")))?;

    Response::from_json(&items)
}
