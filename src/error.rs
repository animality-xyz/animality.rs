extern crate native_tls;
extern crate serde_json;

use native_tls::HandshakeError;

use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::net::TcpStream;
use std::io;

#[derive(Debug)]
pub struct InvalidAnimalError;

impl StdError for InvalidAnimalError {}

impl Display for InvalidAnimalError {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Invalid animal name provided.")
  }
}

/// If the client receives an invalid HTTP status code, the error would be this.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,norun
/// extern crate animality;
/// use animality::{Animality, Animal, RequestError};
/// 
/// let client = Animality::new("your user token");
/// 
/// let error = client.image(Animal::Dog).unwrap_err();
/// 
/// match error {
///   RequestError::HttpError(http_err) => {
///     eprintln!("Received {} - {}", http_err.status_code(), http_err.message());
///   },
/// 
///   _ => eprintln!("Whoops! {}", error),
/// }
/// ```
#[derive(Debug)]
pub struct HttpError {
  inner_status_code: u16,
  inner_message: String,
}

impl HttpError {
  pub(crate) const fn new(inner_status_code: u16, inner_message: String) -> Self {
    Self { inner_status_code, inner_message }
  }

  /// Retrieves the HTTP error status code.
  /// 
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::{Animality, Animal, RequestError};
  /// 
  /// let client = Animality::new("your user token");
  /// 
  /// let error = client.image(Animal::Dog).unwrap_err();
  /// 
  /// match error {
  ///   RequestError::HttpError(http_err) => {
  ///     eprintln!("Received {}", http_err.status_code());
  ///   },
  /// 
  ///   _ => eprintln!("Whoops! {}", error),
  /// }
  /// ```
  pub const fn status_code(&self) -> u16 {
    self.inner_status_code
  }

  /// Retrieves the HTTP error status message.
  /// 
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::{Animality, Animal, RequestError};
  /// 
  /// let client = Animality::new("your user token");
  /// 
  /// let error = client.image(Animal::Dog).unwrap_err();
  /// 
  /// match error {
  ///   RequestError::HttpError(http_err) => {
  ///     eprintln!("{}", http_err.message());
  ///   },
  /// 
  ///   _ => eprintln!("Whoops! {}", error),
  /// }
  /// ```
  #[inline(always)]
  pub fn message(&self) -> &str {
    &self.inner_message
  }
}

impl StdError for HttpError {
  #[inline(always)]
  fn description(&self) -> &str {
    self.message()
  }
}

impl Display for HttpError {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} - {}", self.status_code(), self.message())
  }
}

impl Into<u16> for HttpError {
  #[inline(always)]
  fn into(self) -> u16 {
    self.status_code()
  }
}

/// The error whenever the client fails to initiate a request (for various reasons)
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
/// let error = client.image(Animal::Dog).unwrap_err();
/// eprintln!("error! {}", error);
/// ```
#[derive(Debug)]
pub enum RequestError {
  /// Whenever the client receives an invalid status code.
  HttpError(HttpError),

  /// Whenever [`native_tls::TlsConnector::new`] fails.
  CreatingTlsConnector(native_tls::Error),

  /// Whenever [`std::net::TcpStream::connect`] fails.
  CreatingTcpStream(io::Error),

  /// Whenever [`native_tls::TlsConnector::connect`] fails.
  Handshake(HandshakeError<TcpStream>),

  /// Whenever [`native_tls::TlsStream::write_all`] fails.
  WritingRequest(io::Error),
  
  /// Whenever [`native_tls::TlsStream::read_to_string`] fails.
  ReadingResponse(io::Error),

  /// Whenever [`serde_json::from_str`] fails.
  ParsingJsonResponse(serde_json::Error),
}

impl RequestError {
  pub(crate) const fn inner(&self) -> &dyn StdError {
    match self {
      Self::HttpError(err) => err,
      Self::CreatingTlsConnector(err) => err,
      Self::CreatingTcpStream(err) => err,
      Self::Handshake(err) => err,
      Self::WritingRequest(err) => err,
      Self::ReadingResponse(err) => err,
      Self::ParsingJsonResponse(err) => err,
    }
  }
}

impl StdError for RequestError {
  #[inline(always)]
  fn cause(&self) -> Option<&dyn StdError> {
    Some(self.inner())
  }
}

impl Display for RequestError {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "RequestError({})", self.inner())
  }
}