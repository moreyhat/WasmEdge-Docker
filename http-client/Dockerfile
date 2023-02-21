FROM --platform=$BUILDPLATFORM rust:1.67 as build

WORKDIR /usr/src/http-client
COPY src ./src
COPY Cargo.toml .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/http-client.wasm http-client.wasm

FROM scratch
COPY --link --from=build /usr/src/http-client/http-client.wasm http-client.wasm
ENTRYPOINT [ "http-client.wasm" ]