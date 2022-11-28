FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 as chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build our project
RUN cargo build --release

FROM debian:11-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/nlpf-2 nlpf-2
COPY settings settings
COPY templates templates

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd appuser \
    && useradd -g appuser appuser \
    && chown -R appuser:appuser /app

USER appuser

CMD ["./nlpf-2"]
