# Setup chef
FROM rustlang/rust:nightly AS chef
RUN cargo install cargo-chef

# Build
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build image and run tests
RUN cargo build --release --bin ethsbell-rewrite --features=ws
RUN rm ./src/main.rs && rm ./src/lib.rs
COPY . .
RUN touch src/main.rs && touch src/lib.rs
RUN cargo test --release --features=ws
RUN cargo build --release --bin ethsbell-rewrite --features=ws

FROM ubuntu
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl make build-essential && rm -rf /var/lib/apt/lists
RUN update-ca-certificates
RUN apt update && rm -rf /var/apt/lists
ARG GITHUB_SHA
ENV GITHUB_SHA=$GITHUB_SHA
ARG GITHUB_REPOSITORY
ENV GITHUB_REPOSITORY=$GITHUB_REPOSITORY

# Rust
COPY --from=builder /app/target/release/ethsbell-rewrite .
COPY --from=builder /app/def.d def.d
COPY --from=builder /app/def.json .

# Frontend
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs
RUN mkdir frontend
COPY package*.json .posthtmlrc .
COPY frontend frontend
RUN npm i
RUN npm run build -- --no-cache
COPY frontend/favicon.ico frontend-dist

COPY templates templates

CMD ["./ethsbell-rewrite"]
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -o /dev/null -w "%{http_code}\n" http://localhost:8000/api/v1/coffee | grep 418