use crate::model::Model;
use anyhow::Result;

mod v2;

pub async fn parse_model(name: String, tag: String) -> Result<Model> {
    v2::parse_model(name, tag).await
}
