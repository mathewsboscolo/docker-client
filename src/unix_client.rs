use std::{error::Error, future::Future};

use hyper::{body::to_bytes, Body, Client, Request, Response};
use hyper_unix_connector::{UnixClient, Uri};
use serde::Deserialize;

#[derive(Deserialize)]
struct DockerVersion {
    #[serde(rename = "ApiVersion")]
    api_version: String
}

impl DockerVersion {
    pub async fn fetch_docker_api_version(socket: &str) -> Result<Self, Box<dyn Error>> {
        let uri = Uri::new(
            socket,
            DockerEndpoint::Version.path()
        );
        
        let request = Request::get(uri).body(Body::empty()).map_err(|err| {
            println!("Failed to build the request: {}", err);
            err
        })?;

        let client = Client::builder().build(UnixClient);

        let response = client.request(request).await.map_err(|err| {
            println!("Failed to send the request: {}", err);
            err
        })?;

        let body_bytes = to_bytes(response.into_body()).await.map_err(|err| {
            println!("Failed to read the response body: {}", err);
            err
        })?;

        let parsed: DockerVersion = serde_json::from_slice(&body_bytes).map_err(|err| {
            eprintln!("Failed to parse to json: {}", err);
            err
        })?;

        Ok(parsed)
    }
}

pub enum DockerEndpoint {
    Ping,
    Version,
    ListContainers,
    ListImages,
}

impl DockerEndpoint {
    pub fn path(&self) -> &str {
        match self {
            DockerEndpoint::Version => "/version",
            DockerEndpoint::Ping => "/_ping",
            DockerEndpoint::ListContainers => "/containers/json",
            DockerEndpoint::ListImages => "images/json",
        }
    }
}

pub struct DockerResponse {
    pub status: u16,
    pub response: Response<Body>,
}

impl DockerResponse {
    fn new(status: u16, response: Response<Body>) -> Self {
        DockerResponse { status, response }
    }

    async fn parse<T>(self) -> Result<T, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let body_bytes = to_bytes(self.response.into_body()).await.map_err(|err| {
            println!("Failed to read the response body: {}", err);
            err
        })?;

        let parsed: T = serde_json::from_slice(&body_bytes).map_err(|err| {
            eprintln!("Failed to parse to json: {}", err);
            err
        })?;

        Ok(parsed)
    }
}

pub trait DockerClient {
    type Client;

    fn ping(self) -> impl Future<Output = Result<String, Box<dyn Error>>>;
}

#[derive(Debug, Clone)]
pub struct UnixDockerClient {
    pub socket: String,
    pub api_version: String,
}

impl UnixDockerClient {
    pub async fn new(socket: &str) -> Result<Self, Box<dyn Error>> {
        let docker_version = DockerVersion::fetch_docker_api_version(socket).await
            .map_err(|err| {
                println!("Could not find the current docker version");
                err
            })?;

        Ok(UnixDockerClient {
            socket: socket.to_string(),
            api_version: "v".to_owned() + &docker_version.api_version
        })
    }

    // make the fetch method more abstract by allowing the HTTP method and body/header to be specified
    // this opens up the possibility of including options for tasks like listing, for example
    pub async fn fetch(self, endpoint: DockerEndpoint) -> Result<Response<Body>, Box<dyn Error>> {
        let raw_uri = "/".to_owned() + &self.api_version + endpoint.path();
        let uri = Uri::new(
            self.socket,
            &raw_uri,
        );

        let request = Request::get(uri).body(Body::empty()).map_err(|err| {
            println!("Failed to build the request: {}", err);
            err
        })?;

        let client = Client::builder().build(UnixClient);
        let response = client.request(request).await.map_err(|err| {
            println!("Failed to send the request: {}", err);
            err
        })?;

        Ok(response)
    }

    pub async fn fetch_parsed<T>(self, endpoint: DockerEndpoint) -> Result<T, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let response = self.fetch(endpoint).await?;

        let status = response.status().as_u16();
        let response = DockerResponse::new(status, response);

        let parsed = response.parse().await?;
        Ok(parsed)
    }

    pub async fn fetch_as_string(self, endpoint: DockerEndpoint) -> Result<String, Box<dyn Error>> {
        let response = self.fetch(endpoint).await?;

        let body_bytes = to_bytes(response.into_body()).await.map_err(|err| {
            println!("Failed to read the response body: {}", err);
            err
        })?;

        Ok(String::from_utf8(body_bytes.to_vec())?)
    }
}

impl DockerClient for UnixDockerClient {
    type Client = UnixDockerClient;

    async fn ping(self) -> Result<String, Box<dyn Error>> {
        let ping_response = self
            .fetch_as_string(DockerEndpoint::Ping)
            .await
            .map_err(|err| {
                println!("Failed to ping Docker daemon: {}", err);
                err
            })?;
        Ok(ping_response)
    }
}
