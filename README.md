# Tabby Registry

This repository uses ollama library to download models. You must add `TABBY_DOWNLOAD_HOST=registry.ollama.ai` in tabby's environment in order to use this registry. 

## Passive maintenance notice
From Tabby 0.12 onward (starting with [commit](https://github.com/TabbyML/tabby/commit/1d1edfec6ebc09526399abee428f9406efd31b87)), it is possible to connect your Tabby instance to Ollama. Furthermore, you can enable automatic model downloading with the `TABBY_OLLAMA_ALLOW_PULL=1` environment variable. Ollama will handle chat templates and stop words for you; however, you still need to set a prompt template for FIM. If you want to use models from Ollama, consider deploying Ollama (as a sidecar, for example) and connecting Tabby to it.

With that in mind, I am moving this repository into passive maintenance. You can open issues and pull requests to add new models. While I will review them, I will not actively develop this repository further. It's possible that I will add new FIM models solely to store prompt templates for them.

## Warning

The model list is auto generated. Some models may not work properly.

If a model haves _instruct_ in tag name you should prefer use it. Chat templates are set for instruct models and can work incorrectly with others.

# Models

This repository generates a registry for [Tabby](https://github.com/TabbyML/tabby) but uses [ollama library](https://ollama.com/library) instead a official manual filled [registry](https://github.com/TabbyML/registry-tabby). The models in registry must be included in allow list: [meta/models.yaml](meta/models.yaml).

## Model naming
All models named using this pattern:

    <name>:<tag>

Where:
  * `<name>` - name of model
  * `<tag>` - tag as specified by ollama library. Note: it is required

Example:

    deepseek-coder:6.7b-base-q8_0
    ^              ^
    Name           Tag

## Current models

| Name                         | Chat? | Completion? | Comments                                                          |
|------------------------------|-------|-------------|-------------------------------------------------------------------|
| starcoder                    | No    | Yes         |                                                                   |
| starcoder2                   | Yes   | Yes         |                                                                   |
| deepseek-coder               | Yes   | Yes         |                                                                   |
| gemma                        | Yes   | No          |  Requires Tabby 0.10+ or after [commit](https://github.com/TabbyML/tabby/pull/1805/commits/4b5217533ee842ec3f4709dae9337a91969f3c41)   |
| codegemma                    | Yes   | Yes         | Requires Tabby 0.10+ or after [commit](https://github.com/TabbyML/tabby/pull/1805/commits/4b5217533ee842ec3f4709dae9337a91969f3c41) |
| mistral                      | Yes   | No          |                                                                   |
| mixtral                      | Yes   | No          |                                                                   |
| wizardcoder                  | Yes   | No          |                                                                   |
| codellama                    | Yes   | Yes         | Prefer `code` tags for completion, `instruct` for chat            |
| openhermes                   | Yes   | No          |                                                                   |
| nous-hermes2                 | Yes   | No          |                                                                   |
| nous-hermes2-mixtral         | Yes   | No          |                                                                   |
| codeqwen                     | Yes   | Yes         | `chat` for chat, `code` for completion, others for chat           |
| llama3                       | Yes   | No          | Unsupported stop words, so looping will happen                    |
| llama3-chatqa                | Yes   | No          | Short answers, does not use code blocks even if you ask to do so  |
| codestral                    | Yes   | Yes         |                                                                   |
| deepseek-coder-v2            | Yes   | Yes         | `base` for completion, `instruct` for chat                            |