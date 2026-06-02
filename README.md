# Pola

**Note**, this project is currently in active early-stage development.

**Pola Core** is the foundational engine for an intelligent, background-first file sorting application. It is designed to automatically monitor directories set by the user, extract text from incoming documents, and use TF-IDF and Naive Bayes algorithms to instantly categorize and move files to their correct module workspaces.


## Architecture

Pola is split into two crates:

- **`pola-core`** — the core intelligence library. Handles text normalisation, word frequency analysis, TF-IDF classification, and SQLite persistence. Licensed under MIT.
- **`pola-cli`** — the command-line application. Handles file watching, user configuration, and orchestrates the core pipeline.

## Current Status

| Module | Status |
|---|---|
| Text parser (`parser.rs`) | Done |
| File reader | Planned |
| TF-IDF Classifier | Planned |
| SQLite store | Planned |
| CLI & file watcher | Planned |

## License

`pola-core` is open source under the [MIT License](pola-core/LICENSE).
