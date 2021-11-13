# Frontend
FROM ubuntu as frontend
WORKDIR /app
RUN apt-get update && apt-get install -y curl make build-essential && rm -rf /var/lib/apt/lists
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs
RUN mkdir frontend
COPY package*.json .
COPY frontend frontend
RUN npm i
RUN npm run build -- --no-cache

# Setup chef
FROM rustlang/rust:nightly AS chef
WORKDIR /app
RUN cargo install cargo-chef

# Build dependencies
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
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
RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl && rm -rf /var/lib/apt/lists
RUN update-ca-certificates
RUN apt update && rm -rf /var/apt/lists
ARG GITHUB_SHA
ENV GITHUB_SHA=$GITHUB_SHA
ARG GITHUB_REPOSITORY
ENV GITHUB_REPOSITORY=$GITHUB_REPOSITORY
COPY --from=frontend /app/frontend-dist frontend-dist
COPY frontend/favicon.ico frontend-dist
COPY --from=builder /app/target/release/ethsbell-rewrite .
COPY --from=builder /app/def.d def.d
COPY --from=builder /app/def.json .
COPY templates templates
CMD ["./ethsbell-rewrite"]
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -o /dev/null -w "%{http_code}\n" http://localhost:8000/api/v1/coffee | grep 418