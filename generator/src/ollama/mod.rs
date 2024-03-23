use crate::model::Model;
use anyhow::Result;

mod v2;

pub async fn parse_model(name: &str, tag: &str) -> Result<Model> {
    v2::parse_model(name, tag).await
}
