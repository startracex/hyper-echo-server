use crate::utils;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use hyper::header::HeaderValue;
use hyper::{Request, Response};
use json::{object, stringify_pretty, JsonValue};

pub async fn echo(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let method = req.method().clone();
    let path = req.uri().to_string();
    let headers = req.headers().clone();

    let whole_body = req.collect().await?.to_bytes();

    let mut header = JsonValue::new_object();

    for (name, value) in headers.iter() {
        let _ = header.insert(name.as_str(), value.to_str().unwrap().to_string());
    }

    let mut echo_object = object! {
        "method": method.to_string(),
        "path": path,
        "header": header,
    };

    match String::from_utf8(whole_body.to_vec()) {
        Ok(s) => {
            if !s.is_empty() {
                let _ = echo_object.insert("body", s);
            }
        }
        _ => (),
    };

    let echo = stringify_pretty(echo_object, 4);
    let mut res = Response::new(utils::full(echo));
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));

    Ok(res)
}
