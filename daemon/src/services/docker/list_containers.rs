use bollard::container::ListContainersOptions;

use super::init::docker;







pub async fn list_containers() -> Result<Vec<String>, String> {
    let docker = docker();
    let containers = docker.list_containers(Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    })).await
    .map_err(|e| e.to_string())?;

    let mut container_ids = Vec::new();
    for container in &containers {
        println!("Container ID: {:?}", container.ports);
        if let Some(id) = &container.id {
            container_ids.push(id.clone());
        }
    }
    
    Ok(container_ids)
}