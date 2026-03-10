mod responses;

use std::net::SocketAddrV4;
use std::{net::Ipv4Addr, sync::Arc};
use std::convert::Infallible;
use tokio::sync::{RwLock, broadcast};
use futures_util::StreamExt;
use hyper::{ 
  Method, 
  Request,
  Response,
  Result,
  body::{Bytes, Frame, Incoming}, 
};
use http_body_util::{
  BodyExt,
  StreamBody,
  combinators::BoxBody
};

use crate::util::ogg_headers::Headers;
use responses::{
  default_response,
  stream_response,
  four_oh_four,
  prepare_header_stream,
  wait_for_ogg_headers,
  apply_cors,
  cors_preflight_response
};

pub async fn handle_request(
    req: Request<Incoming>,
    tx: broadcast::Sender<Bytes>,
    ogg_header: Arc<RwLock<Option<Headers>>>,
    mount: Arc<String>
) -> Result<Response<BoxBody<Bytes, Infallible>>> {
  match (req.method(), req.uri().path()) {
    (&Method::GET, path) if path == mount.as_ref() => {
      let rx = tx.subscribe();
      let stream = tokio_stream::wrappers::BroadcastStream::new(rx)
        .filter_map(
          |msg| 
          async move { msg.ok() }
        )
        .map(
          |chunk| 
            Ok::<Frame<Bytes>, Infallible>( 
              Frame::data( chunk)
            )
        )
        .take_while(
          |res|
            futures_util::future::ready(res.is_ok())
        );

      // wait for headers to be populated
      let headers = wait_for_ogg_headers(&ogg_header).await;
      // prepend the ogg headers to the stream body
      let stream = prepare_header_stream(headers)
        .chain(stream);

      let body: BoxBody<Bytes, Infallible> = BodyExt::boxed(StreamBody::new(stream));
      let mut res = stream_response(body);
      apply_cors(&req, &mut res);
      Ok(res)
    },

    (&Method::GET, "/" | "/index.html") => {

      let html = format!("<html><body><a href=\"{}{mount}\">Audio Stream</a></body></html>",
        std::net::IpAddr::V4(Ipv4Addr::LOCALHOST).to_string());
      let body = http_body_util::Full::new(Bytes::from(html)).boxed();
      let mut res = default_response(body);
      apply_cors(&req, &mut res);
      Ok(res)
    },
    (&Method::OPTIONS, _) => {
      let mut res = cors_preflight_response();
      apply_cors(&req, &mut res);
      Ok(res)
    }
    _ => {
      let mut res = four_oh_four();
      apply_cors(&req, &mut res);
      Ok(res)
    }
  }
}

