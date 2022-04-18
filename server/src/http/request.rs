use super::method::Method;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Debug, Result as FmtResult};
use std::error::Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

// implement some trait on the byte array type
// we need to pass in a concrete type 
impl TryFrom<&[u8]> for Request {

    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // we can use the unimplemented!() macro if we don't want to implement this
        unimplemented!();
    }
}


// represent different parsing errors we might encounter
pub enum ParseError {
    // a request is not supported
    InvalidRequest,
    // when request is not UTF8 encoded
    InvalidEncoding,
    // when the protocol is not 1.1 
    InvalidProtocol,
    // when the method doesn't exist
    InvalidMethod,
}

// create a trait to display message
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            // when request is not UTF8 encoded
            Self::InvalidEncoding => "Invalid Encoding",
            // when the protocol is not 1.1 
            Self::InvalidProtocol => "Invalid Protocol",
            // when the method doesn't exist
            Self::InvalidMethod => "Invalid Method",

        }
    }
}

// implement Display trait to format error
impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// implement Debug trait to format error
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
