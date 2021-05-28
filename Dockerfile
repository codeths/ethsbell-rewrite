FROM rustlang/rust:nightly AS builder
ARG GITHUB_SHA
ENV GITHUB_SHA=$GITHUB_SHA
ARG GITHUB_REPOSITORY
ENV GITHUB_REPOSITORY=$GITHUB_REPOSITORY
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){println!(\"dummy\");}" > ./src/main.rs
RUN cargo build --release --bin ethsbell-rewrite --features=ws
RUN rm ./src/main.rs
COPY . .
RUN touch src/main.rs
RUN cargo build --release --bin ethsbell-rewrite --features=ws

FROM ubuntu
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists
RUN update-ca-certificates
COPY --from=builder /app/def.json /app/target/release/ethsbell-rewrite ./
COPY --from=builder /app/frontend ./frontend
CMD ["./ethsbell-rewrite"]