mod statements;

use std::{net::TcpListener, path::PathBuf, sync::Arc};

use axum::{
    debug_handler,
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use sqlite::ConnectionWithFullMutex;
use statements::{state_create_table, state_download, state_upload};
use tower_http::cors::{Any, CorsLayer};

const DB_PATH: &str = "./databases/.sqlite";
const DB_DIR: &str = "./databases";

struct Core {
    db: ConnectionWithFullMutex,
}

#[tokio::main]
async fn main() {
    let predefined_db_dir = PathBuf::from(DB_DIR);
    if !predefined_db_dir.exists() {
        std::fs::create_dir_all(predefined_db_dir).unwrap();
    }

    let db_path = match std::env::var("DATABASE_PATH") {
        Ok(p) => PathBuf::from(&p),
        Err(_) => PathBuf::from(DB_PATH),
    };
    let connection = sqlite::Connection::open_with_full_mutex(&db_path).unwrap();
    connection.execute(state_create_table()).unwrap();

    let shared = Arc::new(Core { db: connection });

    let layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let router = Router::new()
        .route("/", post(upload))
        .route("/:id", get(download))
        .route("/health", get(health))
        .layer(layer)
        .with_state(shared);

    axum::Server::from_tcp(TcpListener::bind("0.0.0.0:8080").expect("Failed to listen."))
        .expect("Failed to listen.")
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn health() -> String {
    "Hello, yadon.".to_string()
}

#[debug_handler]
async fn upload(State(core): State<Arc<Core>>, body: String) -> String {
    let ulid = ulid::Ulid::new().to_string();
    core.db.execute(state_upload(&ulid, &body)).unwrap();
    ulid
}

#[debug_handler]
async fn download(State(core): State<Arc<Core>>, Path(id): Path<String>) -> String {
    let mut body = String::new();
    core.db
        .iterate(state_download(&id), |pairs| {
            for &(column, value) in pairs.iter() {
                match column {
                    "body" => body = value.unwrap().to_owned(),
                    "timestamp" => println!("{}", value.unwrap()),
                    _ => {}
                }
            }
            true
        })
        .unwrap();
    body
}
