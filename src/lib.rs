use worker::*;

mod clients;
mod routes;
mod usecases;

#[event(fetch)]
pub async fn router(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/search", routes::search::search)
        .run(req, env)
        .await
}
