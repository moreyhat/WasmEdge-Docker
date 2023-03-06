# Fibonacci sequence WASM sample
A fibonacci sequence example for WASM.  
The app calculates fibonacci number of index n passed as an argument.

## Build
Build a docker image for WASM
```shell
cd fibonacci-sequence
docker build --platform=wasi/wasm32 -t fibonacci-sequence .
```

## Run
Run the container with `io.containerd.wasmedge.v1` runtime.  
Pass an index `n` as an argument.
```shell
docker run --rm --platform=wasi/wasm32 --runtime=io.containerd.wasmedge.v1 --name=fibonacci-sequence fibonacci-sequence:latest n
```