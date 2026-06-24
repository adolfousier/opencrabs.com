FROM rust:1.94-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown

# Use pre-built trunk binary to avoid dependency hell
# Downloads arch-appropriate binary (aarch64 on ARM64, x86_64 on amd64)
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      curl -fsSL https://github.com/trunk-rs/trunk/releases/download/v0.21.7/trunk-aarch64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin; \
    else \
      curl -fsSL https://github.com/trunk-rs/trunk/releases/download/v0.21.7/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin; \
    fi

# Install mdbook for docs
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      curl -fsSL https://github.com/rust-lang/mdBook/releases/download/v0.4.44/mdbook-v0.4.44-aarch64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin; \
    else \
      curl -fsSL https://github.com/rust-lang/mdBook/releases/download/v0.4.44/mdbook-v0.4.44-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin; \
    fi

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY index.html style.css Trunk.toml ./

# Build landing page (WASM)
RUN trunk build --release

# Build docs (mdbook)
COPY docs/ docs/
RUN cd docs && mdbook build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY --from=builder /app/src/root-files/ /usr/share/nginx/html/
COPY --from=builder /app/docs/book/ /usr/share/nginx/html/docs/
COPY src/docker/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 19283
