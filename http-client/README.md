# HTTP Client WASM sample
A simple http client example for WASM.  
The app retrieves its global IP address, and display it on the terminal.

## Build
Build a docker image for WASM
```shell
cd http-client
docker build --platform=wasi/wasm32 -t http-client-wasm .
```

## Run
Run the container with `io.containerd.wasmedge.v1` runtime.
```shell
docker run --rm --platform=wasi/wasm32 --runtime=io.containerd.wasmedge.v1 --name=http-client-wasm http-client-wasm:latest
```