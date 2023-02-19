# Echo with WASM
A simple echo application with WASM.

## Build
Build a docker image for WASM
```shell
cd echo
docker buildx build --platform=wasi/wasm32 -t echo-wasm .
```

## Run
Run the container with `io.containerd.wasmedge.v1` runtime.
```shell
docker run --rm --platform=wasi/wasm32 --runtime=io.containerd.wasmedge.v1 --name=wasm-echo echo-wasm:latest Awesome, Wasm+Docker!
```