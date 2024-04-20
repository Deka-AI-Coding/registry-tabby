use crate::model::Model;
use anyhow::Result;
use reqwest::Client;

mod manifest;

/// URL for model downloading
const REGISTRY_URL_BASE: &str = "https://registry.ollama.ai:443/v2/library";
/// URL for user available info
const PUBLIC_URL_BASE: &str = "https://ollama.com/library";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Missing license")]
    MissingLicense,
    #[error("Missing model")]
    MissingModel,
    #[error("Incorrect digest format")]
    Sha256Format,
}

fn get_manifest_url(name: &str, tag: &str) -> String {
    format!("{}/{}/manifests/{}", REGISTRY_URL_BASE, name, tag)
}

fn get_layer_url(name: &str, layer: &manifest::Layer) -> String {
    format!("{}/{}/{}", REGISTRY_URL_BASE, name, layer.blob_subpath())
}

fn get_provider_url(name: &str) -> String {
    format!("{}/{}", PUBLIC_URL_BASE, name)
}

async fn get_manifest(name: &str, tag: &str) -> Result<manifest::Manifest> {
    let url = get_manifest_url(name, tag);
    println!("Handling: {}", url);
    let client = Client::new();
    client
        .get(url)
        .header("Accept", manifest::HEADER_ACCEPT_VALUE)
        .send()
        .await?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn parse_model(name: String, tag: String) -> Result<Model> {
    let manifest = get_manifest(&name, &tag).await?;

    let license = manifest.license_layer();
    let model = manifest.model_layer().ok_or(Error::MissingModel)?;
    let checksum = model.checksum().ok_or(Error::Sha256Format)?;

    let license_url = if let Some(layer) = license {
        get_layer_url(&name, layer)
    } else {
        get_provider_url(&name)
    };

    Ok(Model {
        name: format!("{}:{}", name, tag),
        license_url,
        provider_url: get_provider_url(&name),
        sha256: checksum,

        urls: vec![get_layer_url(&name, model)],

        // To be filled by the yaml
        prompt_template: None,
        chat_template: None,
    })
}
