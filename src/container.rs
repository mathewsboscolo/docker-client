use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::error::Error;

use crate::unix_client::{DockerEndpoint, UnixDockerClient};

trait QueryParameters {
    fn to_query_string(&self) -> Option<String>;
}

impl QueryParameters for ListContainersParameters {
    fn to_query_string(&self) -> Option<String> {
        serde_urlencoded::to_string(self).ok()
    }
}

impl UnixDockerClient {
    pub async fn list_containers(
        &self,
        list_containers: Option<ListContainersParameters>,
    ) -> Result<Vec<Container>, Box<dyn Error>> {
        let path = DockerEndpoint::ListContainers.path();
        let query_string = list_containers
            .and_then(|params| params.to_query_string())
            .map(|qs| format!("{}?{}", path, qs))
            .unwrap_or_else(|| path.to_string());

        let containers: Vec<Container> = self
            .fetch_parsed(query_string, None) // is the string type inference really necessary?
            .await?;

        Ok(containers)
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ListContainersParameters {
    pub all: Option<bool>,
    pub limit: Option<i32>,
    pub size: Option<bool>,
    pub filters: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    #[serde(rename = "Id")]
    container_id: String,

    #[serde(rename = "Names")]
    names: Vec<String>,

    #[serde(rename = "Image")]
    image: String,
}
