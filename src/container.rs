use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::unix_client::{DockerEndpoint, UnixDockerClient};

impl UnixDockerClient {
    
    pub async fn list_containers(self) -> Result<Vec<Container>, Box<dyn Error>> {
        let containers: Vec<Container> = self.fetch_parsed(DockerEndpoint::ListContainers).await.map_err(|error| {
            println!("Could not list containers: {}", error);
            error
        })?;
        Ok(containers)
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    #[serde(rename = "Id")]
    container_id: String,

    #[serde(rename = "Names")]
    names: Vec<String>,

    #[serde(rename = "Image")]
    image: String,

    #[serde(rename = "ImageId")]
    image_id: String
}