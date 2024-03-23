use std::env::args;
use std::fs::File;
use std::process::exit;

use anyhow::Result;
use model::Model;

use crate::model::ModelInput;

mod model;
mod ollama;

fn convert(model_input: &ModelInput) -> anyhow::Result<Vec<Model>> {
    model_input
        .tags
        .iter()
        .map(|tag| ollama::parse_model(&model_input.name, tag))
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args().collect::<Vec<_>>();

    if args.len() != 3 {
        println!("Usage: {} <input> <output>", args[0]);
        exit(1);
    }

    let input_file = File::open(&args[1])?;
    let output_file = File::create(&args[2])?;

    let inputs: Vec<ModelInput> = serde_yaml::from_reader(input_file)?;
    let processed: Vec<Vec<Model>> = inputs.iter().map(convert).collect::<Result<_>>()?;
    let flatten: Vec<Model> = processed.into_iter().flatten().collect();

    serde_json::to_writer_pretty(output_file, &flatten)?;

    Ok(())
}
