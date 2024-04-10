# Tabby Registry

This repository uses ollama library to download models. You must add `TABBY_DOWNLOAD_HOST=registry.ollama.ai` in tabby's environment in order to use this registry. 

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
| starcoder2                   | No    | Yes         |                                                                   |
| deepseek-coder               | Yes   | Yes         |                                                                   |
| gemma                        | Yes   | No          |                                                                   |
| codegemma                    | Yes   | Yes         |                                                                   |
| mistral                      | Yes   | No          |                                                                   |
| mixtral                      | Yes   | No          |                                                                   |
| wizardcoder                  | Yes   | No          |                                                                   |
| codellama                    | Yes   | Yes         | Prefer `code` tags for completion, `instruct` for chat            |
| openhermes                   | Yes   | No          |                                                                   |
| nous-hermes2                 | Yes   | No          |                                                                   |
| nous-hermes2-mixtral         | Yes   | No          |                                                                   |

