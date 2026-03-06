FROM rust:1.85-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY index.html style.css Trunk.toml ./

RUN trunk build --release

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY src/docker/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 19283
