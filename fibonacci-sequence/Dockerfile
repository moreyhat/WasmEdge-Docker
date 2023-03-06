FROM --platform=$BUILDPLATFORM rust:1.67 as build

WORKDIR /usr/src/fibonacci-sequence
COPY src ./src
COPY Cargo.toml .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/fibonacci-sequence.wasm fibonacci-sequence.wasm

FROM scratch
COPY --link --from=build /usr/src/fibonacci-sequence/fibonacci-sequence.wasm fibonacci-sequence.wasm
ENTRYPOINT [ "fibonacci-sequence.wasm" ]