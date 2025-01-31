use docker_client::{container::ListContainersParameters, unix_client::{DockerClient, DockerEndpoint, UnixDockerClient}};

const SUCCESS_PING_RESPONSE: &str = "OK";
const DEFAULT_SOCKET: &str = "/.docker/run/docker.sock";

async fn create_docker_client() -> UnixDockerClient {
    UnixDockerClient::new(DEFAULT_SOCKET).await.expect("Failed to create Docker client")
}

#[tokio::test]
async fn fetch_as_string_integration() {
    let docker_client = create_docker_client().await;
    let ping_response: String = docker_client.fetch_as_string(DockerEndpoint::Ping.path(), None).await.unwrap();

    assert_eq!(ping_response, SUCCESS_PING_RESPONSE);
}

#[tokio::test]
async fn ping_request_integration() {
    let docker_client = create_docker_client().await;
    let ping = docker_client.ping().await.unwrap();

    assert_eq!(ping, SUCCESS_PING_RESPONSE);
}

#[tokio::test]
async fn check_api_version_integration() {
    let docker_client = create_docker_client().await;
    assert!(!docker_client.api_version.is_empty());
}

// it would be cool a testcontainers-rs integration with it
#[tokio::test]
async fn list_containers_integration() {
    let client = create_docker_client().await;
    let containers = &client.list_containers(None).await.unwrap();

    assert!(!containers.is_empty());
}

#[tokio::test]
async fn list_containers_integration_including() {
    let client = create_docker_client().await;
    let list_containers_params = ListContainersParameters {
        all: Some(true),
        limit: Some(10),
        size: Some(true),
        filters: Some(String::from(r#"{"status":["running"]}"#)),
    };

    let containers = &client.list_containers(Some(list_containers_params)).await.unwrap();
    assert!(!containers.is_empty());
}