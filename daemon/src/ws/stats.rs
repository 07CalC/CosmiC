use std::time::Duration;

use axum::response::IntoResponse;
use sysinfo::{Components, Disks, System};
use tokio::time::interval;
use axum::extract::ws::{WebSocketUpgrade, WebSocket, Message};
use chrono::Utc;




pub async fn stats_ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_stats)
}

async fn handle_stats(mut socket: WebSocket) {
    let mut system = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let disk_data = disks.iter().map(|disk| {
        let name = disk.name().to_string_lossy().to_string();
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;
        serde_json::json!({
            "name": name,
            "totalSpace": total_space,
            "availableSpace": available_space,
            "usedSpace": used_space
        })
    }).collect::<Vec<_>>();
    let mut interval = interval(Duration::from_millis(2000));

    while let Some(_) = interval.tick().await.into() {
        system.refresh_all();
        let cpu = system.global_cpu_usage();
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        let components =  Components::new_with_refreshed_list();
        let cpu_temp = components.iter().find_map(|c| {
            if c.label().contains("Core") {
                c.temperature().map(|temp| temp as f64)
            } else {
                None
            }
        });
        let cpu_temp = cpu_temp.unwrap_or(-1.0);
        let load = System::load_average();
        let uptime = System::uptime();
        let stats = serde_json::json!({
            "cpuUsage": cpu,
            "memory": {
                "total": total_memory,
                "used": used_memory
            },
            "disk": disk_data,
            "loadAverage": {
                "oneMinute": load.one,
                "fiveMinutes": load.five,
                "fifteenMinutes": load.fifteen
            },
            "uptime": uptime,
            "cpuTemperature": cpu_temp,
            "timestamp": Utc::now().timestamp()
        });

        if socket
            .send(Message::Text(stats.to_string().into()))
            .await
            .is_err()
        {
            break;
        }
    }
}