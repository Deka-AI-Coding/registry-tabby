use std::env::args;
use std::fs::File;
use std::process::exit;

use anyhow::Result;
use model::Model;

use crate::model::ModelInput;

mod model;
mod ollama;

async fn convert(model_input: &ModelInput) -> anyhow::Result<Vec<Model>> {
    let mut out = vec![];

    for tag in &model_input.tags {
        let mut tmp_model = ollama::parse_model(&model_input.name, tag).await?;

        // Override
        model_input.chat_template.as_ref().inspect(|m| {
            tmp_model.chat_template.get_or_insert(m.to_string());
        });

        model_input.prompt_template.as_ref().inspect(|m| {
            tmp_model.prompt_template.get_or_insert(m.to_string());
        });

        out.push(tmp_model)
    }

    Ok(out)
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
    let mut outputs = vec![];

    for input in &inputs {
        let mut out_models = convert(input).await?;
        outputs.append(&mut out_models);
    }

    serde_json::to_writer_pretty(output_file, &outputs)?;

    Ok(())
}
