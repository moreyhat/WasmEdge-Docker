FROM --platform=$BUILDPLATFORM rust:1.67 as build

WORKDIR /usr/src/simple-queue
COPY src ./src
COPY Cargo.toml .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/simple-queue.wasm simple-queue.wasm

FROM scratch
COPY --link --from=build /usr/src/simple-queue/simple-queue.wasm simple-queue.wasm
ENTRYPOINT [ "simple-queue.wasm" ]