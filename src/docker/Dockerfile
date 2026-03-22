FROM rust:1.94-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk mdbook

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY index.html style.css Trunk.toml ./
COPY docs/ docs/

RUN trunk build --release
RUN cd docs && mdbook build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY --from=builder /app/docs/book /usr/share/nginx/html/docs
COPY src/docker/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 19283
