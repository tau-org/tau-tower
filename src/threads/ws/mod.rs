use hyper::{Request, Response, StatusCode};
use tokio::sync::broadcast;
use std::sync::{Arc, Mutex};

use hyper::body::Bytes;
use tokio_tungstenite::{accept_hdr_async, WebSocketStream};
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::time::Duration;

use crate::util::{Headers, validate_tags, validate_header, Credentials};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::Instant;
use crate::threads::LOG_TIMEOUT;

const TIMEOUT: Duration = Duration::from_millis(50);

/// Creates a WebSocket receiver listening to the sender of the ogg opus stream.
/// Appending the ogg opus blocks to a producer/consumer object.
pub async fn thread(
    tx: broadcast::Sender<Bytes>,
    src_addr: SocketAddr,
    credentials: Credentials,
    header: Arc<Mutex<Headers>>
) {
  let server = match TcpListener::bind(src_addr).await {
    Ok(s) => s,
    Err(e) => {
      eprintln!("Could not bind to source address: {e}");
      return;
    }
  };

  loop {
    match server.accept().await {
      Ok((stream, addr)) => {
        match accept_hdr_async(stream, |req: &Request<()>, res: hyper::Response<()>|
          validate_headers(req, res, &credentials)
        ).await {
          Ok(mut ws_stream) => {
            receive_data(&mut ws_stream, header.clone(), tx.clone()).await;
          },
          Err(e) => {
            eprintln!("Handshake failed from {addr}: {e}");
          }
        }
      },
      Err(e) => {
        eprintln!("{e}");
        tokio::time::sleep(TIMEOUT).await;
      }
    }
  }
}

fn validate_headers(req: &Request<()>, res: hyper::Response<()>, credentials: &Credentials) -> Result<Response<()>, Response<Option<String>>> {
  let username = req.headers()
    .get("username")
    .and_then(|u| u.to_str().ok());

  let password = req.headers()
    .get("password")
    .and_then(|pw| pw.to_str().ok());

  if username.is_none() || password.is_none() {
    return Err(
      Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Some("Missing credentials".to_string()))
        .unwrap()
    )
  }

  if !credentials.validate(username, password) {
    return Err(
      Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Some("Credentials do not match".to_string()))
        .unwrap()
    );
  }

  Ok(res)
}

async fn receive_data(ws_stream: &mut WebSocketStream<TcpStream>, header: Arc<Mutex<Headers>>, tx: broadcast::Sender<Bytes>) {
  let mut temp_headers: (Option<Bytes>, Option<Bytes>) = (None, None);
  let mut headers_parsed = false;
  let mut last_log = Instant::now();
  'connections: while let Some(msg) = ws_stream.next().await  {
    let page = match msg {
      Ok(m) => m.into_data(),
      Err(e) => {
        eprintln!("Unrecognized message: {e}");
        break 'connections;
      }
    };

    if !headers_parsed { // short circuit if headers have already been parsed.
      if let Ok(head) = validate_header(page.clone()) { 
        temp_headers.0 = head; 
      } 
      if let Ok(tags) = validate_tags(page.clone()) { 
        temp_headers.1 = tags; 
      } 
      if let (Some(head), Some(tags)) = &temp_headers 
        && let Ok(mut h) = header.lock() 
        && let None = h.headers {
        h.prepare_headers(&(head, tags));
        headers_parsed = true;
      }
    }

    if let Err(e) = tx.send(page) && last_log.elapsed() > LOG_TIMEOUT {
      eprintln!("could not open client stream: {e}"); 
      last_log = Instant::now();
    }
  }
}


