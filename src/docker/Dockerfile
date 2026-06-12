FROM rust:1.94-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown

# Use pre-built trunk binary to avoid dependency hell
RUN curl -fsSL https://github.com/trunk-rs/trunk/releases/download/v0.21.7/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY index.html style.css Trunk.toml ./

RUN trunk build --release

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY src/docker/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 19283
