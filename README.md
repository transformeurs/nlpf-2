# NLPF-2

## Stack

- Rust (Axum & Askama)
- Neo4j
- S3 (MinIO)
- Redis
- Docker
- GitHub Actions

## Get started

- Install Rust: https://rustup.rs/
- Run `cargo install cargo-watch` (for hot reloading the project)
- Run `docker compose up -d`
- Run `cargo watch -x run` and start develop 🚀

NB: the server will be listening to `127.0.0.1:8080` by default.

## Resources

- Axum
  - Official doc: https://docs.rs/axum/0.5.17/axum/index.html
  - Official examples: https://github.com/tokio-rs/axum/tree/0.5.x/examples
- Neo4j
  - General doc: https://neo4j.com/docs/getting-started/current/
  - Query cheat sheet: https://neo4j.com/docs/cypher-refcard/current/
- Neo4rs documentation: https://docs.rs/neo4rs/0.5.9/neo4rs/index.html
- Askama (HTML templates) syntax: https://djc.github.io/askama/template_syntax.html
