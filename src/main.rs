use std::time::*;
use sysinfo::{System, Disks};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use netdev::interface;


fn convert_to_gb(bytes: u64) -> f32 {
    let bytes = bytes as f32 / 1024.0 / 1024.0 / 1024.0;
    return (bytes * 100.0).round() / 100.0;
}


#[tokio::main]
async fn main(){
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await.unwrap();
    println!("ws listening on localhost:8000");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let socket_stream = accept_async(stream).await.unwrap();
            let (mut write, _) = socket_stream.split();

            

            let mut sys = System::new_all();
            let mut prev_rx = 0;
            let mut prev_tx = 0;
            loop{
                sys.refresh_all();
                let cpu = sys.global_cpu_usage();
                let total_memory = convert_to_gb(sys.total_memory());
                let used_memory = convert_to_gb(sys.used_memory());
                let use_memory_percentage = (used_memory as f32 / total_memory as f32) * 100.0;


                let mut total_disk_space = 0.0;
                let mut available_disk_space = 0.0;

                let disk = Disks::new_with_refreshed_list();

                for disk in disk.iter() {
                    total_disk_space += convert_to_gb(disk.total_space());
                    available_disk_space += convert_to_gb(disk.available_space());
                }

                let used_disk_space = total_disk_space - available_disk_space;

                // let use_disk_percentage = (used_disk_space as f32 / total_disk_space as f32) * 100.0;

                
                let network_interface = interface::get_default_interface().unwrap();

                let mut total_rx = 0;
                let mut total_tx = 0;

                let message = json!({
                    "cpu": cpu,
                    "total_memory": total_memory,
                    "used_memory": used_memory,
                    "use_memory_percentage": use_memory_percentage,
                    "total_disk_space": total_disk_space,
                    "used_disk_space": used_disk_space,
                    "available_disk_space": available_disk_space,
                });

                if write.send(tokio_tungstenite::tungstenite::Message::Text(message.to_string().into())).await.is_err() {
                    break;
                }

                tokio::time::sleep(Duration::from_secs(2)).await;

            }
        });
    }
}