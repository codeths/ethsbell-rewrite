# Setup chef
FROM docker.io/rustlang/rust:nightly AS chef
RUN cargo install cargo-chef

# Build
FROM chef AS planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build image and run tests
COPY LICENSE.html def.json* def.example.json ./
RUN cp -n def.example.json def.json
COPY src src
COPY tests tests
COPY templates templates
RUN cargo test --release --features=ws
RUN cargo build --release --bin ethsbell-rewrite --features=ws

FROM docker.io/ubuntu AS frontend
RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl make build-essential && rm -rf /var/lib/apt/lists
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs
WORKDIR /app
COPY package*.json .posthtmlrc .browserslistrc ./
RUN npm i
COPY frontend frontend
RUN npm run build -- --no-cache
COPY frontend/favicon.ico frontend-dist

FROM docker.io/ubuntu
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl && rm -rf /var/lib/apt/lists
RUN update-ca-certificates
ARG GITHUB_SHA
ENV GITHUB_SHA=$GITHUB_SHA
ARG GITHUB_REPOSITORY
ENV GITHUB_REPOSITORY=$GITHUB_REPOSITORY

# Rust
COPY --from=builder /app/target/release/ethsbell-rewrite .
# COPY def.json* def.example.json ./
COPY --from=builder /app/def.json ./

# Frontend
COPY --from=frontend /app/frontend-dist frontend-dist

CMD ["./ethsbell-rewrite"]
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -o /dev/null -w "%{http_code}\n" http://localhost:8000/api/v1/coffee | grep 418