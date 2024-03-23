use serde::Deserialize;

pub const HEADER_ACCEPT_VALUE: &str = "application/vnd.docker.distribution.manifest.v2+json";

#[derive(Deserialize, Debug, PartialEq)]
pub enum LayerMediaType {
    #[serde(rename = "application/vnd.ollama.image.model")]
    Model,
    #[serde(rename = "application/vnd.ollama.image.license")]
    License,
    #[serde(rename = "application/vnd.ollama.image.template")]
    Template,
    #[serde(rename = "application/vnd.ollama.image.params")]
    Params,
}

#[derive(Deserialize, Debug)]
pub struct Layer {
    #[serde(rename = "mediaType")]
    media_type: LayerMediaType,

    digest: String,
}

impl Layer {
    pub fn checksum(&self) -> Option<String> {
        let Some((mark, sum)) = self.digest.split_once(':') else {
            return None;
        };

        if mark == "sha256" {
            Some(sum.into())
        } else {
            None
        }
    }

    pub fn blob_subpath(&self) -> String {
        format!("blobs/{}", self.digest)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ManifestMediaType {
    #[serde(rename = "application/vnd.docker.distribution.manifest.v2+json")]
    V2,
}

#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    _version: u32,
    #[serde(rename = "mediaType")]
    _media_type: ManifestMediaType,

    layers: Vec<Layer>,
}

impl Manifest {
    pub fn license_layer(&self) -> Option<&Layer> {
        self.layers
            .iter()
            .find(|l| l.media_type == LayerMediaType::License)
    }

    pub fn model_layer(&self) -> Option<&Layer> {
        self.layers
            .iter()
            .find(|l| l.media_type == LayerMediaType::Model)
    }
}
