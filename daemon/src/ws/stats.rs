use std::time::Duration;

use axum::response::IntoResponse;
use sysinfo::System;
use tokio::time::interval;
use axum::extract::ws::{WebSocketUpgrade, WebSocket, Message};
use chrono::Utc;




pub async fn stats_ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_stats)
}

async fn handle_stats(mut socket: WebSocket) {
    let mut system = System::new_all();
    let mut interval = interval(Duration::from_millis(2000));

    while let Some(_) = interval.tick().await.into() {
        system.refresh_all();

        let cpu = system.global_cpu_usage();
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();

        let stats = serde_json::json!({
            "cpuUsage": cpu,
            "memory": {
                "total": total_memory,
                "used": used_memory
            },
            "timestamp": Utc::now().timestamp()
        });

        if socket
            .send(Message::Text(stats.to_string().into()))
            .await
            .is_err()
        {
            break; // client disconnected
        }
    }
}