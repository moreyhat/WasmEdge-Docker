# Web based Simple Queue sample
A simple queue providing web interface for WASM.
The app add enqueue and dequeue methods.

## Build
Build a docker image for WASM.
```shell
cd simple-queue
docker build --platform wasi/wasm32 -t simple-queue . 
```

## Run
Run the container with `io.containerd.wasmedge.v1` runtime.
```shell
docker run --rm -dp 1234:1234 --name simple-queue --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 simple-queue
```

## Enqueue/Dequeue
Put a message:
```shell
$ curl -d "This is the first message." -X POST http://127.0.0.1:1234
Successfully added a message.
```

Get a message:
```shell
$ curl http://127.0.0.1/1234
[{"message": "This is the first message."}]
```