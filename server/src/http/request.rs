use super::method::Method;
use std::convert::TryFrom;

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

    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // we can use the unimplemented!() macro if we don't want to implement this
        unimplemented!();
    }
}