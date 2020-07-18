use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::str::{self, Utf8Error};
use std::fmt::{Display,Result as FmtResult, Formatter};

pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buffer: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

// when we implement `TryFrom`, the compiler will auto-generate code that implements `TryInto` trait for type T
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // to => GET /search?name=example&sort=1 HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Swap Utf8Error to InvalidEncodingError
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {

}
