# 🧑‍🔬 Tabby Registry

## Completion models (`--model`)

We recommend using

* For **1B to 3B models**, it's advisable to have at least **NVIDIA T4, 10 Series, or 20 Series GPUs**, or **Apple Silicon** like the M1.
* For **7B to 13B models**, we recommend using **NVIDIA V100, A100, 30 Series, or 40 Series GPUs**.

We have published benchmarks for these models on https://leaderboard.tabbyml.com for Tabby's users to consider when making trade-offs between quality, licensing, and model size.

| Model ID | License |
| -------- | ------- |
| [Deka-AI-Coding/StarCoder-1B](https://huggingface.co/bigcode/starcoderbase-1b) | [BigCode-OpenRAIL-M](https://huggingface.co/spaces/bigcode/bigcode-model-license-agreement) |
| [Deka-AI-Coding/StarCoder-3B](https://huggingface.co/bigcode/starcoderbase-3b) | [BigCode-OpenRAIL-M](https://huggingface.co/spaces/bigcode/bigcode-model-license-agreement) |
| [Deka-AI-Coding/StarCoder-7B](https://huggingface.co/bigcode/starcoderbase-7b) | [BigCode-OpenRAIL-M](https://huggingface.co/spaces/bigcode/bigcode-model-license-agreement) |
| [Deka-AI-Coding/CodeLlama-7B](https://huggingface.co/codellama/CodeLlama-7b-hf) | [Llama 2](https://github.com/facebookresearch/llama/blob/main/LICENSE) |
| [Deka-AI-Coding/CodeLlama-13B](https://huggingface.co/codellama/CodeLlama-13b-hf) | [Llama 2](https://github.com/facebookresearch/llama/blob/main/LICENSE) |
| [Deka-AI-Coding/DeepseekCoder-1.3B](https://huggingface.co/deepseek-ai/deepseek-coder-1.3b-base) | [Deepseek License](https://github.com/deepseek-ai/deepseek-coder/blob/main/LICENSE-MODEL) |
| [Deka-AI-Coding/DeepseekCoder-6.7B](https://huggingface.co/deepseek-ai/deepseek-coder-6.7b-base) | [Deepseek License](https://github.com/deepseek-ai/deepseek-coder/blob/main/LICENSE-MODEL) |


## Chat models (`--chat-model`)

To ensure optimal response quality, and given that latency requirements are not stringent in this scenario, we recommend using a model with at least 3B parameters.

| Model ID | License |
| -------- | ------- |
| [Deka-AI-Coding/WizardCoder-3B](https://huggingface.co/WizardLM/WizardCoder-3B-V1.0) | [BigCode-OpenRAIL-M](https://huggingface.co/spaces/bigcode/bigcode-model-license-agreement) |
| [Deka-AI-Coding/Mistral-7B](https://huggingface.co/mistralai/Mistral-7B-v0.1) | [Apache 2.0](https://choosealicense.com/licenses/apache-2.0/) |
| [Deka-AI-Coding/DeepseekCoder-1.3B-Instruct](https://huggingface.co/deepseek-ai/deepseek-coder-1.3b-instruct) | [Deepseek License](https://github.com/deepseek-ai/deepseek-coder/blob/main/LICENSE-MODEL) |
| [Deka-AI-Coding/DeepseekCoder-6.7B-Instruct](https://huggingface.co/deepseek-ai/deepseek-coder-6.7b-instruct) | [Deepseek License](https://github.com/deepseek-ai/deepseek-coder/blob/main/LICENSE-MODEL) |
| [Deka-AI-Coding/Gemma-7B-it](https://huggingface.co/google/gemma-7b-it) | [Gemma Terms of Use](https://ai.google.dev/gemma/terms) |
| [Deka-AI-Coding/Gemma-7B](https://huggingface.co/google/gemma-7b) | [Gemma Terms of Use](https://ai.google.dev/gemma/terms) |
| [Deka-AI-Coding/Test-Ollama-Gemma-7B](https://huggingface.co/google/gemma-7b) | [Gemma Terms of Use](https://ai.google.dev/gemma/terms) |
