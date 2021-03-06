use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Debug, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::QueryString;

// to use lifetimes we must make Request generic
#[derive(Debug)]
pub struct Request<'buf> {
    // this is a reference with 'a lifetime
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        // use as_ref to be able to return an Option wrapped around &QueryString
        self.query_string.as_ref()
    }
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

// implement some trait on the byte array type
// we need to pass in a concrete type
// to use the lifetime generic we must declare is on the impl.
// lifetimes are a powerful tool for the Rust compiler to guarantee memory safety
// lifetime params don't allow us to choose how long a value lives but that some references
// point to the same memory and are expected to share the same lifetime
impl <'buf> TryFrom<&'buf [u8]> for Request<'buf> {

    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    // when we call this we make sure the lifetime is not de-allocated from the buffer
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

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
        
        // if the option is some it will convert it to an Ok result
        // we're also doing some variable shadowing here meaning `request` is no longer usable
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // check requests are valid and extract the bytes needed 
        // path is mutable because we later reassign a value to it in the match pattern
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        
        // parse the method string
        let method: Method = method.parse()?;

        // assume its None and only match i if Some
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            // plus one byte
            query_string = Some(QueryString::from(&path[i +1..]));
            path = &path[..i];
        }
    
        Ok(Self {
            path: path,
            query_string,
            method,
        })
    }
}
// what it would look to write lifetimes explicitly
// fn get_next_word<'a, 'b>(request: &'a str, b: &'b str) -> Option<(&'a str, &'b str)>{
fn get_next_word(request: &str) -> Option<(&str, &str)>{

    // chars is an iterator that returns either Next or None
    // and to get the index use enumerate which returns a tuple
    for (i, c) in request.chars().enumerate() {

        // check if the char is a space or a carriage return
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
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

// allows us to use MethodError from method.rs
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}