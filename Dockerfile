FROM rustlang/rust:nightly AS builder
RUN apt update && apt install -y npm && rm -rf /var/apt/lists
ARG GITHUB_SHA
ENV GITHUB_SHA=$GITHUB_SHA
ARG GITHUB_REPOSITORY
ENV GITHUB_REPOSITORY=$GITHUB_REPOSITORY
WORKDIR /app
COPY Cargo.toml Cargo.lock package.json ./
RUN npm i
RUN mkdir src && echo "fn main(){println!(\"dummy\");}" > ./src/main.rs && touch src/lib.rs
RUN mkdir src/bin && echo "fn main(){println!(\"dummy\");}" > ./src/bin/bell_mkschema.rs
RUN cargo build --release --bin ethsbell-rewrite --features=ws
RUN rm ./src/main.rs && rm ./src/lib.rs
COPY . .
RUN touch src/main.rs && touch src/lib.rs
# This is here so the build fails if the tests do.
RUN cargo test --release --features=ws
RUN npm run build
RUN cargo build --release --bin ethsbell-rewrite --features=ws

FROM ubuntu
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl && rm -rf /var/lib/apt/lists
RUN update-ca-certificates
COPY --from=builder /app/def.json /app/target/release/ethsbell-rewrite ./
COPY --from=builder /app/def.d/* ./def.d/
COPY --from=builder /app/frontend-dist ./frontend-dist
COPY --from=builder /app/frontend/favicon.ico /app/frontend-dist/favicon.ico
COPY --from=builder /app/templates ./templates
CMD ["./ethsbell-rewrite"]
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -o /dev/null -w "%{http_code}\n" http://localhost:8000/api/v1/coffee | grep 418