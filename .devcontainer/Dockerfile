FROM mcr.microsoft.com/vscode/devcontainers/base

ENV RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:$PATH

# Install packages
RUN apt-get update && apt-get install -y ca-certificates build-essential fish zsh libssl-dev pkg-config faketime && rm -rf /var/lib/apt/lists

# Install Rust
RUN set -eux; \
	\
	url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
	wget "$url"; \
	chmod +x rustup-init; \
	./rustup-init -y --no-modify-path --default-toolchain nightly; \
	rm rustup-init; \
	chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
	rustup toolchain install --allow-downgrade nightly -c rustfmt; \
	rustup --version; \
	cargo --version; \
	rustc --version;

# Install npm
RUN apt-get update && apt-get install npm -y && rm -rf /var/lib/apt/lists
# Install xo
RUN npm i -g xo
# Copy time zone
COPY ./localtime /etc/localtime