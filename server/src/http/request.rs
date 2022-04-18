use super::method::Method;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Debug, Result as FmtResult};
use std::error::Error;
use std::str;
use std::str::Utf8Error;

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
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // // if this is a utf8 error it will tell you
        // // otherwise it will return the custom error type
        // // common Rust pattern: match on a result and if its an error, return `e`
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }

        // the above is equivalent to this because we implemented From<Utf8Error>
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
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

// allows us to use from_utf8 in our TryFrom implementation for error handling
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Error for ParseError {}
