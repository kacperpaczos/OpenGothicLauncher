# ogl-network

`ogl-network` handles all external HTTP interactions. It is decoupled from the rest of the application so that testing can cleanly mock network requests.

## Key Responsibilities

1. **Release Discovery (`releases`)**: Fetches the GitHub releases HTML page for `Try/OpenGothic` and parses release tags to build deterministic download URLs.
2. **Secure Downloading (`downloads`)**: Streams large archive files asynchronously in chunks.
3. **Integrity Verification**: As chunks are streamed, it computes a SHA-256 hash using the `sha2` crate. If an expected hash is provided, the download is validated and corrupted files are deleted.

## Architecture

Heavily relies on `tokio` for async I/O and `reqwest` for the HTTP client. All functions return `Result` types wrapped by `thiserror`, making network failures easy to propagate and display in the GUI/CLI.
