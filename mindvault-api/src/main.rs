use axum::extract::State;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tracing::info;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("--- Starting MindVault Service ---");

    let server_up_since = Utc::now().format("%d/%m/%y %H:%M").to_string().to_string();
    let mind_vault_router = Router::new()
        .route("/", get(root_handler).with_state(server_up_since));

    let listener = TcpListener::bind("127.0.0.1:4500").await?;
    info!("Listening on {}", &listener.local_addr()?);
    axum::serve(listener, mind_vault_router).await?;

    info!("--- MindVault Service Stopped ---");
    Ok(())
}

async fn root_handler(State(server_up_since): State<String>) -> Html<String> {
    let resp = format!("App is running! Since <b>{} UTC</b>", server_up_since);
    Html(resp)
}