use bollard::Docker;




pub fn docker() -> Docker {
    Docker::connect_with_defaults().expect("Failed to connect to Docker")
}