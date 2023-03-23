# MySQL database client sample
A simple WASM based REST API.

## Build
Build a docker image for WASM.
```shell
cd mysql-client
docker compose build
```

## Run
Run the container.
```shell
docker compose up -d
```

## API request
Put an order:
```shell
$ curl -d "{\"order_id\": 1,\"account_id\": 1,\"product_id\": 1,\"quantity\": 1}" -X POST http://localhost:8080/orders
{"status":"success"}
$ curl -d "{\"order_id\": 2,\"account_id\": 100,\"product_id\": 15,\"quantity\": 3}" -X POST http://localhost:8080/orders
{"status":"success"}
```

Get orders:
```shell
$ curl http://localhost:8080/orders
[{"order_id":1,"account_id":1,"product_id":1,"quantity":1},{"order_id":2,"account_id":100,"product_id":15,"quantity":3}]
```

## Clean up
Clean up the containers:
```shell
docker compose down
```