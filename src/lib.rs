//! # animality.rs
//! 
//! A simple Rust API wrapper that generates images & facts of any animal.
//! 
//! # Installation
//! 
//! Add this to your `Cargo.toml file's dependencies:`
//! ```toml
//! animality = "1.0.0"
//! ```
//! 
//! # Blocking request
//! 
//! ```rust,norun
//! extern crate animality;
//! use animality::{Animality, Animal};
//! 
//! fn main() {
//!   let client = Animality::new("your token here");
//!   
//!   // request with the `Animal` enum
//!   let dog_image = client.image(Animal::Dog).unwrap();
//!   let dog_fact = client.fact(Animal::Dog).unwrap();
//!   
//!   // request from a string (case-insensitive) 
//!   let cat: Animal = "cat".parse().unwrap();
//!   let cat_image = client.image(cat).unwrap();
//!   let cat_fact = client.fact(cat).unwrap();
//! }
//! ```
//! 
//! # Async request
//! 
//! ```rust,norun
//! extern crate animality;
//! extern crate tokio;
//! 
//! use animality::{Animality, Animal, RequestError};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), RequestError> {
//!   let client = Animality::new("your token here");
//!   
//!   // request with the `Animal` enum
//!   let dog_image = client.image_async(Animal::Dog).await?;
//!   let dog_fact = client.fact_async(Animal::Dog).await?;
//!   
//!   // request from a string (case-insensitive) 
//!   let cat: Animal = "cat".parse().unwrap();
//!   let cat_image = client.image_async(cat).await?;
//!   let cat_fact = client.fact_async(cat).await?;
//! 
//!   Ok(())
//! }
//! ```

mod error;
mod animal;
pub use error::*;
pub use animal::*;

extern crate serde_json;
extern crate native_tls;

use native_tls::{TlsConnector, TlsStream};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::fmt::Display;

type ApiResponse = Result<serde_json::Map<String, serde_json::Value>, RequestError>;

/// The Animality HTTPS client. This struct handles every request sent and received to the [Animality API](https://animality.xyz).
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,norun
/// extern crate animality;
/// use animality::Animality;
/// 
/// let animality = Animality::new("your user token");
/// ```
#[must_use]
pub struct Animality {
  headers: String,
}

impl Animality {
  /// Creates the animality HTTP client. This requires a token to use.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::Animality;
  /// 
  /// let animality = Animality::new("your user token");
  /// ```
  pub fn new<K: AsRef<str> + Display>(key: &K) -> Self {
    let headers = format!("
      Content-Type: application/json\r\n\
      Accept: application/json\r\n\
      Authorization: Bearer {}\r\n\r\n", key
    );
    
    Self { headers }
  }

  fn parse_response(mut stream: TlsStream<TcpStream>) -> ApiResponse {
    let mut raw_response = String::new();
    
    if let Err(err) = stream.read_to_string(&mut raw_response) {
      return Err(RequestError::ReadingResponse(err));
    }
    
    let mut line_iterator = raw_response.lines();
      
    let mut first_line = line_iterator
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
      .get(raw_response.find("\r\n\r\n").expect("Cannot retrieve body from response.") + 4..)
      .expect("Cannot retrieve body from response.");
  
    match serde_json::from_str(response) {
      Err(err) => Err(RequestError::ParsingJsonResponse(err)),
      Ok(json) => match json {
        serde_json::Value::Object(json_obj) => Ok(json_obj),
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
      Err(err) => Err(RequestError::Handshake(err)),
      Ok(s) => Ok(s),
    }?;

    let headers = format!("GET {} HTTP/1.0\r\n{}", path, self.headers);

    if let Err(err) = stream.write_all(headers.as_bytes()) {
      return Err(RequestError::ReadingResponse(err));
    }

    Self::parse_response(stream)
  }

  /// Fetches a random animal image from the Animality API.
  /// This request blocks the current process. To use an async version, use [`Animality::image_async`].
  ///
  /// # Errors
  /// 
  /// Returns a [`RequestError`] if the client fails to initiate a request with the API,
  /// if the client receives a non-OK HTTP response, or
  /// if the client cannot parse the API response as JSON.
  /// 
  /// # Panics
  /// 
  /// This function panics if the API returns a response with no body,
  /// or if the JSON response structure from the API is invalid.
  /// 
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::{Animality, Animal};
  /// 
  /// let client = Animality::new("your user token");
  /// 
  /// // request with the `Animal` enum
  /// let dog_image = client.image(Animal::Dog).unwrap();
  /// 
  /// // request from a string (case-insensitive) 
  /// let cat: Animal = "cat".parse().unwrap();
  /// let cat_image = client.image(cat).unwrap();
  /// ```
  #[must_use]
  pub fn image(&self, animal: Animal) -> Result<String, RequestError> {
    let response = self.request(format!("/img/{}", animal.as_str()))?;

    match response.get("link").expect("Cannot find image.") {
      serde_json::Value::String(s) => Ok(s.to_owned()),
      _ => panic!("The JSON 'link' value is not a string."),
    }
  }

  /// Fetches a random animal fact from the Animality API.
  /// This request blocks the current process. To use an async version, use [`Animality::fact_async`].
  ///
  /// # Errors
  /// 
  /// Returns a [`RequestError`] if the client fails to initiate a request with the API,
  /// if the client receives a non-OK HTTP response, or
  /// if the client cannot parse the API response as JSON.
  /// 
  /// # Panics
  /// 
  /// This function panics if the API returns a response with no body,
  /// or if the JSON response structure from the API is invalid.
  /// 
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::{Animality, Animal};
  /// 
  /// let client = Animality::new("your user token");
  /// 
  /// // request with the `Animal` enum
  /// let dog_fact = client.fact(Animal::Dog).unwrap();
  /// 
  /// // request from a string (case-insensitive) 
  /// let cat: Animal = "cat".parse().unwrap();
  /// let cat_fact = client.fact(cat).unwrap();
  /// ```
  #[must_use]
  pub fn fact(&self, animal: Animal) -> Result<String, RequestError> {
    let response = self.request(format!("/fact/{}", animal.as_str()))?;

    match response.get("fact").expect("Cannot find fact.") {
      serde_json::Value::String(s) => Ok(s.to_owned()),
      _ => panic!("The JSON 'fact' value is not a string."),
    }
  }

  /// Fetches a random animal image from the Animality API asynchronously.
  /// To use the blocking version, use [`Animality::image`].
  ///
  /// # Errors
  /// 
  /// Returns a [`RequestError`] if the client fails to initiate a request with the API,
  /// if the client receives a non-OK HTTP response, or
  /// if the client cannot parse the API response as JSON.
  /// 
  /// # Panics
  /// 
  /// This function panics if the API returns a response with no body,
  /// or if the JSON response structure from the API is invalid.
  /// 
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::{Animality, Animal};
  /// 
  /// let client = Animality::new("your user token");
  /// 
  /// // request with the `Animal` enum
  /// let dog_image = client.image_async(Animal::Dog).await?;
  /// 
  /// // request from a string (case-insensitive) 
  /// let cat: Animal = "cat".parse().unwrap();
  /// let cat_image = client.image_async(cat).await?;
  /// ```
  #[inline(always)]
  pub async fn image_async(&self, animal: Animal) -> Result<String, RequestError> {
    self.image(animal)
  }

  /// Fetches a random animal fact from the Animality API asynchronously.
  /// To use the blocking version, use [`Animality::fact`].
  ///
  /// # Errors
  /// 
  /// Returns a [`RequestError`] if the client fails to initiate a request with the API,
  /// if the client receives a non-OK HTTP response, or
  /// if the client cannot parse the API response as JSON.
  /// 
  /// # Panics
  /// 
  /// This function panics if the API returns a response with no body,
  /// or if the JSON response structure from the API is invalid.
  /// 
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::{Animality, Animal};
  /// 
  /// let client = Animality::new("your user token");
  /// 
  /// // request with the `Animal` enum
  /// let dog_fact = client.fact_async(Animal::Dog).await?;
  /// 
  /// // request from a string (case-insensitive) 
  /// let cat: Animal = "cat".parse().unwrap();
  /// let cat_fact = client.fact_async(cat).await?;
  /// ```
  #[inline(always)]
  pub async fn fact_async(&self, animal: Animal) -> Result<String, RequestError> {
    self.fact(animal)
  }
}