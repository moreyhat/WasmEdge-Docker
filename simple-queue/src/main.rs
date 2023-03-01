use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use simple_queue::queue::Queue;
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

fn success_response(body: &str) -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        body.to_string(),
    ))
}

fn error_response(body: &str) -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(400)?,
        ReasonPhrase::new("")?,
        body.to_string(),
    ))
}

fn handle_http(req: Request<String>, queue: &mut Queue) -> bytecodec::Result<Response<String>> {
    println!("{:?}", req.request_target());

    match (req.method().as_str(), req.request_target().as_str()) {
        ("POST", "/") => {
            let body = req.body();
            queue.enqueue(body.to_string());
            success_response("Successfully added a message.")
        }
        ("GET", "/") => {
            let message = match queue.dequeue() {
                Some(m) => format!("[{{\"message\": \"{}\"}}]", m),
                None => "[]".to_string(),
            };
            success_response(&message)
        }
        _ => error_response("The method or path is not implemented."),
    }
}

fn handle_client(mut stream: TcpStream, queue: &mut Queue) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req, queue),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    let mut queue = Queue::new();
    loop {
        let _ = handle_client(listener.accept(false)?.0, &mut queue);
    }
}
