use std::io::{Write, Result as IoResult};
use std::fmt::{Display, Formatter, Debug, Result as FmtResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // write response directly to the stream so that we don't need to manage
    // memory allocations on the server 
    // dynamic dispatch: the concrete implementation of the type will happen at runtime - use `dyn`
    // static dispatch: compiler resolves all concrete implementations - use `impl`
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {

        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            f, 
            "HTTP/1.1 {} {} \r\n\r\n {}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}