FROM --platform=$BUILDPLATFORM rust:1.67 as build

WORKDIR /usr/src/client-app
COPY src ./src
COPY Cargo.toml .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/client-app.wasm client-app.wasm

FROM scratch
COPY --link --from=build /usr/src/client-app/client-app.wasm client-app.wasm
EXPOSE 8080
ENTRYPOINT [ "client-app.wasm" ]