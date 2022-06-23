mod error;
mod animal;
pub use error::*;
pub use animal::*;

extern crate serde_json;
extern crate native_tls;

use native_tls::{TlsConnector, TlsStream};
use std::io::{Read, Write};
use std::net::TcpStream;

type ApiResponse = Result<serde_json::Map<String, serde_json::Value>, RequestError>;

#[must_use]
pub struct Animality {
  headers: String,
}

impl Animality {
  pub const fn new(key: &'static str) -> Self {
    let headers = format!("
      Content-Type: application/json\r\n\
      Accept: application/json\r\n\
      Authorization: Bearer {}\r\n\r\n", key
    );
    
    Self { headers }
  }

  fn parse_response(stream: TlsStream<TcpStream>) -> ApiResponse {
    let mut raw_response = String::new();
    
    if let Err(err) = stream.read_to_string(&mut raw_response) {
      return Err(RequestError::ReadingResponse(err));
    }
    
    let mut line_iterator = raw_response.lines();
      
    let first_line = line_iterator
      .next()
      .expect("Cannot retrieve first line of HTTP response.")
      .split_whitespace()
      .skip(1);
    
    let status_code: u16 = first_line
      .next()
      .expect("Cannot retrieve status code from response body.")
      .parse()
      .expect("Cannot parse HTTP status code as number.");

    if status_code >= 400 {
      let message = first_line
        .next()
        .expect("Cannot retrieve status message from response body.")
        .to_string();
    
      return Err(RequestError::HttpError(HttpError::new(status_code, message)));
    }

    let response: &str = raw_response
      .get(raw_response.find("\r\n\r\n") + 4..)
      .expect("Cannot retrieve body from response.");
  
    match serde_json::from_str(response) {
      Err(err) => Err(RequestError::ParsingJsonResponse(err)),
      Ok(json) => match json {
        Object(json_obj) => Ok(json_obj),
        _ => panic!("Expected a JSON object to be returned. Got {}", json),
      }
    }
  }

  fn request(&self, path: String) -> ApiResponse {
    let connector = match TlsConnector::new() {
      Err(err) => Err(RequestError::CreatingTlsConnector(err)),
      Ok(conn) => Ok(conn),
    }?;

    let stream = match TcpStream::connect("api.animality.xyz:443") {
      Err(err) => Err(RequestError::CreatingTcpStream(err)),
      Ok(s) => Ok(s)
    }?;

    let mut stream = match connector.connect("api.animality.xyz", stream) {
      Err(err) => Err(RequestError::ConnectingTcpStream(err)),
      Ok(s) => Ok(s),
    }?;

    let headers = format!("GET {} HTTP/1.0\r\n{}", path, self.headers);

    if let Err(err) = stream.write_all(headers.as_bytes()) {
      return Err(RequestError::ReadingResponse(err));
    }

    Self::parse_response(stream)
  }

  #[must_use]
  pub fn image(&self, animal: Animal) -> Result<String, RequestError> {
    let response = self.request(format!("/img/{}", animal.as_str()))?;

    match response.get("link").expect("Cannot find image.") {
      String(s) => Ok(s),
      _ => panic!("The JSON 'link' value is not a string."),
    }
  }

  #[must_use]
  pub fn fact(&self, animal: Animal) -> Result<String, RequestError> {
    let response = self.request(format!("/fact/{}", animal.as_str()))?;

    match response.get("fact").expect("Cannot find fact.") {
      String(s) => Ok(s),
      _ => panic!("The JSON 'fact' value is not a string."),
    }
  }

  #[inline(always)]
  #[must_use]
  pub async fn image_async(&self, animal: Animal) -> Result<String, RequestError> {
    self.image(animal)
  }

  #[inline(always)]
  #[must_use]
  pub async fn fact_async(&self, animal: Animal) -> Result<String, RequestError> {
    self.fact(animal)
  }
}