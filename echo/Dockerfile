FROM --platform=$BUILDPLATFORM rust:1.67 as build

WORKDIR /usr/src/echo
COPY src ./src
COPY Cargo.toml .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release

FROM scratch
COPY --link --from=build /usr/src/echo/target/wasm32-wasi/release/echo.wasm echo.wasm
ENTRYPOINT [ "echo.wasm" ]