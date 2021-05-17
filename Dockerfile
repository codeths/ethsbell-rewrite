FROM rustlang/rust:nightly AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){println!(\"dummy\");}" > ./src/main.rs
RUN cargo build --release --bin ethsbell-rewrite
COPY . .
RUN cargo build --release --bin ethsbell-rewrite

FROM ubuntu
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists
RUN update-ca-certificates
COPY --from=builder /app/def.json /app/target/release/ethsbell-rewrite ./
COPY --from=builder /app/frontend ./frontend
CMD ["./ethsbell-rewrite"]