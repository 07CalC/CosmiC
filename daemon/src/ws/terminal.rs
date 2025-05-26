use std::process::Stdio;

use axum::{
    extract::{ws::{Message, WebSocket}, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::{StreamExt, SinkExt};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, process::Command};

pub async fn terminal_ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_terminal)
}

pub async fn handle_terminal(mut socket: WebSocket) {
    println!("Terminal WebSocket connection established");
    let mut child = Command::new("/bin/bash")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn shell");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let (mut socket_tx, mut socket_rx) = socket.split();

    tokio::spawn(async move {
        while let Some(msg) = socket_rx.next().await {
            if let Ok(msg) = msg {
                if let Message::Text(data) = msg {
                    println!("Received text message: {}", data);
                    let _ = stdin.write_all(data.as_bytes()).await;
                } else if let Message::Binary(data) = msg {
                    let _ = stdin.write_all(&data).await;
                }
            }
        }
    });

    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            match stdout.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    let _ = socket_tx.send(Message::Binary(axum::body::Bytes::copy_from_slice(&buf[..n]))).await;
                }
                Err(_) => break,
            }
        }
    });
}
