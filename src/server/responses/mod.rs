use std::sync::Arc;
use std::convert::Infallible;
use hyper::{ 
  Request,
  Response,
  StatusCode,
  body::{Bytes, Frame, Incoming}, 
  header::{
    ACCESS_CONTROL_ALLOW_HEADERS,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    ACCESS_CONTROL_EXPOSE_HEADERS,
    ACCESS_CONTROL_MAX_AGE,
    CACHE_CONTROL,
    CONTENT_TYPE,
    ORIGIN,
    VARY
  }
};
use tokio::sync::RwLock;
use futures_util::{Stream, stream};
use http_body_util::{
  BodyExt,
  Empty,
  Full,
  combinators::BoxBody
};
use crate::util::ogg_headers::Headers;

type HttpResponse = Response<BoxBody<Bytes, Infallible>>;

const CORS_REQ_SRC: &str = "http://127.0.0.1:4000"; // localhost asciinema

/// Prepares the Ogg Opus headers, captured from the source stream, to be broadcast on every new
/// listener connection.
pub(super) fn prepare_header_stream(header: Headers) -> impl Stream<Item = core::result::Result<Frame<Bytes>, Infallible>> {
  stream::iter([Ok(Frame::data(header.head)), Ok(Frame::data(header.tags))].into_iter())
}


/// Prevent listeners receiving broken streams, when [`Headers`] struct has not been populated by
/// source stream
pub(super) async fn wait_for_ogg_headers(header: &Arc<RwLock<Option<Headers>>>) -> Headers {
  loop {
    if let Some(h) = header.read().await.as_ref() {
      return h.clone()
    }
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
  }
}

/// 
pub(super) fn stream_response(body: BoxBody<Bytes, Infallible>) -> HttpResponse {
  Response::builder()
  .status(StatusCode::OK)
  .header(CONTENT_TYPE, "audio/ogg; codecs=\"opus\"")
  .header(CACHE_CONTROL, "no_cache")
  // .header(TRANSFER_ENCODING, "chunked")
  // .header(CONNECTION, "keep-alive")
  .header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS_REQ_SRC)
  .body(body)
  .expect("Could not build a stream_response")
}

pub(super) fn cors_preflight_response() -> HttpResponse {
  Response::builder()
    .status(StatusCode::NO_CONTENT)
    .header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS_REQ_SRC)
    .header(ACCESS_CONTROL_ALLOW_HEADERS,"GET, OPTIONS")
    .header(ACCESS_CONTROL_EXPOSE_HEADERS, "Content-Type, Authorization")
    .header(ACCESS_CONTROL_MAX_AGE, "86400")
    .body(BoxBody::new(Empty::<Bytes>::new()))
    .expect("could not build cors_preflight_response")
}

pub(super) fn apply_cors(req: &Request<Incoming>, res: &mut HttpResponse) {
  if let Some(origin) = req.headers().get(ORIGIN) {
    res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
    res.headers_mut().append(VARY, "Origin".parse().expect("could not parse value string"));
  }
}

pub(super) fn default_response(body: BoxBody<Bytes, Infallible>) -> HttpResponse {
  Response::builder()
  .status(StatusCode::OK)
  .header(CONTENT_TYPE, "text/html; charset=utf-8")
  .header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS_REQ_SRC)
  .body(body)
  .expect("could not build default_response")
}

pub(super) fn four_oh_four() -> HttpResponse {
  Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(
      Full::new("NOT_FOUND".into())
        .map_err(|e| match e {})
        .boxed(),
    )
  .expect("could not build 404 response")
}
