FROM rust:1-alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /app

RUN mkdir generate
COPY generate/src ./generate/src
COPY generate/Cargo.toml ./generate/

RUN cd generate && cargo build

COPY ./generate /app/generate
COPY ./feeds.ndjson .

RUN cd generate && cargo run

# ---

FROM nginx:alpine

WORKDIR /app

COPY --from=build /app/generate/output/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/generate/output/index.html ./index.html
