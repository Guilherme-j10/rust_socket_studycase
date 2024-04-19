use axum::{extract::{ws::WebSocket, WebSocketUpgrade}, routing::get, Router};
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = Router::new()
        .route("/", get(handler_ws))
        .route("/test", get(|| async { "ola mundo" }))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler_ws(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            info!("message recived: {:?}", msg);
            msg
        } else {
            info!("Connection closed");
            return;
        };

        if socket.send(msg).await.is_err() {
            info!("Connection closed");
            return;
        }
    }
}

// use axum::{routing::get, Router};
// use socketioxide::{
//     extract::{AckSender, Bin, Data, SocketRef},
//     SocketIo,
// };
// use tower::{Layer, ServiceBuilder};
// use tower_http::cors::{Any, CorsLayer};
// use tracing::info;
// use tracing_subscriber::FmtSubscriber;

// #[cfg(test)]
// use axum_test::TestServer;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     tracing::subscriber::set_global_default(FmtSubscriber::default())?;
//     let app = initialize_server();

//     info!("Starting server...");

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();
//     axum::serve(listener, app).await.unwrap();

//     Ok(())
// }

// async fn on_connect(socket: SocketRef) {
//     info!("socket connected: {}", socket.id);

//     socket.on(
//         "join",
//         |_: SocketRef| async move {
//             info!("Received join:");
//         },
//     );

//     socket.on(
//         "message",
//         |_: SocketRef| async move {
//             info!("Received message");
//         },
//     )
// }

// fn initialize_server() -> Router {
//     let (layer, io) = SocketIo::builder().build_layer();

//     io.ns("/", on_connect);

//     Router::new()
//         .route("/", get(|| async { "hellow word" }))
//         .with_state(io)
//         .layer(
//             ServiceBuilder::new()
//                 .layer(
//                     CorsLayer::permissive()
//                     // CorsLayer::new()
//                     //     .allow_methods(Any)
//                     //     .allow_headers(Any)
//                     //     .allow_origin(Any),
//                 )
//                 .layer(layer),
//         )
// }

// #[cfg(test)]
// fn server_test() -> TestServer {
//     let app = initialize_server();

//     TestServer::new(app).unwrap()
// }

// #[cfg(test)]
// mod test {
//     use crate::server_test;

//     #[tokio::test]
//     async fn test_roo_route() {
//         let server = server_test();

//         let response = server.get("/").await;
//         assert_eq!(response.text(), "hellow word");
//     }
// }
