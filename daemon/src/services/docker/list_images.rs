use bollard::image::ListImagesOptions;

use super::init::docker;






pub async fn list_images() {
    let docker = docker();
    let options = Some(ListImagesOptions::<String>{
        all: true,
        ..Default::default()
    });
    let images = docker.list_images(options).await
        .map_err(|e| e.to_string())
        .expect("Failed to list Docker images");
    for image in images {
        println!("Image ID: {:?}", image.created);
    }
}