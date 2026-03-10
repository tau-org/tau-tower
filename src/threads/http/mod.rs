
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::{net::TcpListener, sync::{RwLock, broadcast}};
use hyper::server::conn::http1;
use std::sync::Arc;
use hyper::body::Bytes;
use crate::server::handle_request;
use crate::util::ogg_headers::Headers;

use super::TIMEOUT;

pub async fn thread(
    ip_addr: impl tokio::net::ToSocketAddrs + std::fmt::Debug,
    tx: broadcast::Sender<Bytes>,
    header: Arc<RwLock<Option<Headers>>>,
    mount: Arc<String>
) -> anyhow::Result<()> {
  let listener = match TcpListener::bind(&ip_addr).await {
    Ok(tl) => tl,
    Err(e) => {
      eprintln!("Could not create TcpListener: {e}");
      eprintln!("{:#?}", ip_addr);
      anyhow::bail!("Could not create TcpListener: {} {:#?}", e, ip_addr);
    }
  };
  let tx_clone = tx.clone();
  let mount = mount.clone();

  loop {
    let (stream, _peer) = match listener.accept().await {
      Ok(sp) => sp,
      Err(e) => {
        eprintln!("Accept error: {e}");
        tokio::time::sleep(TIMEOUT).await; // avoid busy loop
        continue;
      }
    };

    let _ = stream.set_nodelay(true);
    let io = TokioIo::new(stream);
    let tx_inner_clone = tx_clone.clone();
    let header_clone = header.clone();
    let mount_clone = mount.clone();
    tokio::task::spawn(async move {
      if let Err(err) = http1::Builder::new()
        .serve_connection(io, service_fn(move |req| {
          handle_request(req, tx_inner_clone.clone(), header_clone.clone(),  mount_clone.clone())
      }))
        .await
      {
        eprintln!("error serving connection: {}", err);
      }
    });
  }
  #[allow(unreachable_code)]
  anyhow::Ok(())
}
