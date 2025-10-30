use hyper::body::Bytes;

pub struct Credentials {
  pub username: String,
  pub password: String,
  pub broadcast_port: u16
}

impl Credentials {
  pub fn validate(&self, username: Option<&str>, password: Option<&str>, port: Option<u16>) -> bool {
    if (username, password, port) == (
      Some(&self.username),
      Some(&self.password),
      Some(self.broadcast_port)
      ) {
      return true
    }
    false
  }
}

pub struct Headers {pub headers: Option<Bytes>}

impl Headers {
  // pub fn prepare_headers(&mut self, buf: &[Bytes]) {
  //   self.headers = Some(Bytes::copy_from_slice(&[&buf[0][..], &buf[1][..]].concat()))
  // }

  pub fn prepare_headers(&mut self, buf: &(&Bytes, &Bytes)) {
    self.headers = Some(Bytes::copy_from_slice(&[&buf.0[..], &buf.1[..]].concat()))
  }
}

pub fn validate_tags(data: Bytes) -> Result<Option<Bytes>, ()> {
  let n_segs = data[26] as usize;
  let offset = 27+n_segs;
  if data.len() < 27 + 8 { return Err(()) }
  if &data[offset..offset+8] == b"OpusTags" {
    return Ok(Some(data));
  }
  Err(())
}

pub fn validate_header(data: Bytes) -> Result<Option<Bytes>, ()> {
  let n_segs = data[26] as usize;
  let offset = 27+n_segs;
  if data.len() < 27 + 8 { return Err(()) }
  if &data[offset..offset+8] == b"OpusHead" {
    return Ok(Some(data));
  }
  Err(())
}

pub fn validate_bos_and_tags(data: &Bytes) -> core::result::Result<&Bytes, ()> {
  let n_segs = data[26] as usize;
  let offset = 27+n_segs;
  if data.len() < 27 + 8 { return Err(()) }
  if matches!(&data[offset..offset+8], b"OpusTags" | b"OpusHead") {
    return Ok(data);
  }
  Err(())
}
