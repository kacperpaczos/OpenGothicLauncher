# ogl-network

`ogl-network` handles all external HTTP interactions. It is decoupled from the rest of the application so that testing can cleanly mock network requests.

## Key Responsibilities

1. **API Integration (`releases`)**: Uses `reqwest` to interact with the GitHub Releases API (specifically `Try/OpenGothic`). It fetches the JSON payload for the latest releases to find downloadable engine archives.
2. **Secure Downloading (`downloads`)**: Streams large archive files asynchronously in chunks.
3. **Integrity Verification**: As chunks are streamed, it computes a SHA-256 hash using the `sha2` crate. Once the download finishes, it validates the checksum against the expected hash from the GitHub release. If the validation fails, the corrupted file is automatically deleted.

## Architecture

Heavily relies on `tokio` for async I/O and `reqwest` for the HTTP client. All functions return `Result` types wrapped by `thiserror`, making network failures easy to propagate and display in the GUI/CLI.
