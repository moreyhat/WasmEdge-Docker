use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use mysql_async::{prelude::*, Opts, OptsBuilder, Pool, PoolConstraints, PoolOpts};
use serde::{Deserialize, Serialize};
use std::result::Result;
use std::{convert::Infallible, net::SocketAddr};

fn get_url() -> String {
    "mysql://root:password@db:3306/mysql".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    order_id: i32,
    account_id: i32,
    product_id: i32,
    quantity: i32,
}

impl Order {
    fn new(order_id: i32, account_id: i32, product_id: i32, quantity: i32) -> Self {
        Self {
            order_id,
            account_id,
            product_id,
            quantity,
        }
    }
}

async fn handle_request(req: Request<Body>, pool: Pool) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/orders") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream = hyper::body::to_bytes(req).await?;
            let order: Order = serde_json::from_slice(&byte_stream).unwrap();

            "INSERT INTO orders (order_id, account_id, product_id, quantity) VALUES (:order_id, :account_id, :product_id, :quantity)"
            .with(params! {
                "order_id" => order.order_id,
                "account_id" => order.account_id,
                "product_id" => order.product_id,
                "quantity" => order.quantity,
            })
            .ignore(&mut conn)
            .await?;
            drop(conn);
            Ok(response_build("{\"status\":\"success\"}"))
        }

        (&Method::GET, "/orders") => {
            let mut conn = pool.get_conn().await.unwrap();

            let orders = "SELECT * FROM orders"
                .with(())
                .map(&mut conn, |(order_id, account_id, product_id, quantity)| {
                    Order::new(order_id, account_id, product_id, quantity)
                })
                .await?;

            drop(conn);
            Ok(response_build(serde_json::to_string(&orders)?.as_str()))
        }

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn response_build(body: &str) -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "Keep-Alive,User-Agent,Content-Type",
        )
        .body(Body::from(body.to_owned()))
        .unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts = Opts::from_url(&*get_url()).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let constraints = PoolConstraints::new(5, 10).unwrap();
    let pool_opts = PoolOpts::default().with_constraints(constraints);
    let pool = Pool::new(builder.pool_opts(pool_opts));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let make_svc = make_service_fn(|_| {
        let pool = pool.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let pool = pool.clone();
                handle_request(req, pool)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
