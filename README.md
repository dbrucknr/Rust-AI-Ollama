# Rust-AI-Ollama

### Dependencies

- Rust Async Runtime: `cargo add tokio -F tokio/full`
- Futures (types and traits) `cargo add futures tokio-stream -F tokio-stream/sync`
- Base AI Layer: `cargo add rig-core -F rig-core/ollama`
- Backend API: `cargo add axum serde -F serde/derive`

### Resources

- [Tokio](https://tokio.rs/)
- [Rig](https://rig.rs/)
  - [Ollama Example](https://docs.rs/rig-core/latest/rig/providers/ollama/index.html#example)
- [Axum](https://docs.rs/axum/latest/axum/)
