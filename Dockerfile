FROM rustlang/rust:nightly AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM ubuntu
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists
RUN update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/ethsbell-rewrite /app/def.json ./
COPY --from=builder /app/frontend ./frontend
CMD ["./ethsbell-rewrite"]