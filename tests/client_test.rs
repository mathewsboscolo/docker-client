use docker_client::unix_client::{DockerClient, DockerEndpoint, UnixDockerClient};

const SUCCESS_PING_RESPONSE: &str = "OK";
const DEFAULT_SOCKET: &str = "/.docker/run/docker.sock";

#[tokio::test]
async fn fetch_as_string_integration() {
    let docker_client = UnixDockerClient::new(DEFAULT_SOCKET).await.unwrap();
    let ping_response = docker_client.fetch_as_string(DockerEndpoint::Ping).await.unwrap();

    assert_eq!(ping_response, SUCCESS_PING_RESPONSE);
}

#[tokio::test]
async fn ping_request_integration() {
    let docker_client = UnixDockerClient::new(DEFAULT_SOCKET).await.unwrap();
    let ping = docker_client.ping().await.unwrap();

    assert_eq!(ping, SUCCESS_PING_RESPONSE);
}

#[tokio::test]
async fn check_api_version_integration() {
    let docker_client = UnixDockerClient::new(DEFAULT_SOCKET).await.unwrap();
    assert!(!docker_client.api_version.is_empty());
}

// it would be cool a testcontainers-rs integration with it
#[tokio::test]
async fn list_containers_integration() {
    let client = UnixDockerClient::new(DEFAULT_SOCKET).await.unwrap();
    let containers = client.list_containers().await.unwrap();

    assert!(!containers.is_empty());
}
