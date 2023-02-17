FROM --platform=$BUILDPLATFORM rust:1.67 as build

WORKDIR /usr/src/echo
COPY src ./src
COPY Cargo.toml .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/echo.wasm echo.wasm

FROM scratch
COPY --link --from=build /usr/src/echo/echo.wasm echo.wasm
ENTRYPOINT [ "echo.wasm" ]