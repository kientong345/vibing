use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{
    get,
};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use vibing::database::Mp3Database;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Track {
    path: String,
    vibes: Vec<(String, String)>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("can access .env file");

    let server_address = "127.0.0.1:12225";

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is not existed in .env");

    let mp3_db = Arc::new(
        RwLock::new(
            Mp3Database::new(&database_url).await.expect("failed to connect to db")
        )
    );

    let app = Router::new()
        .route("/", get(async || "<h1>Hello</h1>"))
        .route("/tracks", get(get_all_tracks).post(add_track))
        .route("/tracks/identifier", get(get_track).put(set_track).delete(delete_track))
        .route("/vibe_groups", get(get_vibe_groups))
        .route("/vibe_groups/name", get(get_group))
        .with_state(mp3_db);


    let listener = tokio::net::TcpListener::bind(server_address)
        .await
        .expect("cannot create a tcp listener");

    axum::serve(listener, app).await.expect("serve failed");
}

async fn get_all_tracks(State(mp3_db): State<Arc<RwLock<Mp3Database>>>)
-> (StatusCode, Json<Vec<Track>>) {
    let tracks_metadata = mp3_db.read().await.get_all_tracks().await;
    let mut tracks = Vec::new();
    for metadata in tracks_metadata {
        tracks.push(
            Track { path: metadata.path, vibes: metadata.vibes }
        );
    }
    (StatusCode::OK, Json(tracks))
}

async fn add_track(State(mp3_db): State<Arc<RwLock<Mp3Database>>>, track: Json<Track>)
-> Result<(StatusCode, Json<i64>), StatusCode> {
    let path = &track.path;
    let vibes = &track.vibes;
    if let Ok(id) = mp3_db.read().await.add_track_by_path(path).await {
        for (vibe_group, vibe) in vibes {
            let _ = mp3_db.read().await.add_vibe_for(id, vibe_group, vibe).await;
        }
        Ok( (StatusCode::CREATED, Json(id)) )
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Identifier {
    id: i64,
    name: String
}

async fn get_track(State(mp3_db): State<Arc<RwLock<Mp3Database>>>, id: Query<Identifier>)
-> Result<(StatusCode, Json<Track>), StatusCode> {
    let id = id.id.to_string().parse().unwrap();
    let track_metadata = mp3_db.read().await.get_track_by_id(id).await;
    if let Some(metadata) = track_metadata {
        Ok((
            StatusCode::OK,
            Json( Track { path: metadata.path, vibes: metadata.vibes } )
        ))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn set_track(State(mp3_db): State<Arc<RwLock<Mp3Database>>>, id: Query<Identifier>, vibe: Json<(String, String)>)
-> impl IntoResponse {
    
}

async fn delete_track(State(mp3_db): State<Arc<RwLock<Mp3Database>>>, id: Query<Identifier>)
-> impl IntoResponse {
    
}

async fn get_vibe_groups(State(mp3_db): State<Arc<RwLock<Mp3Database>>>)
-> (StatusCode, Json<Vec<String>>) {
    let groups = mp3_db.read().await.get_vibe_groups().await;
    (StatusCode::OK, Json(groups))
}

async fn get_group(State(mp3_db): State<Arc<RwLock<Mp3Database>>>, name: Query<Identifier>)
-> (StatusCode, Json<Vec<Track>>) {
    let mut tracks = Vec::new();
    let vibe_group = name.name.to_string();
    let tracks_metadata = mp3_db.read().await.get_tracks_by_vibe_group(&vibe_group).await;
    for metadata in tracks_metadata {
        tracks.push( Track { path: metadata.path, vibes: metadata.vibes } );
    }
    (StatusCode::OK, Json(tracks))
}
