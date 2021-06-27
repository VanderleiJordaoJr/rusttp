extern crate custom_error;

use custom_error::custom_error;

custom_error! {#[derive(PartialEq,PartialOrd)] pub RequestErrors
    HTTPHeader { request: String } = "Invalid request header: {request}",
    HTTPRequest { method: String } = "Invalid method: {method}",
    UnparsedRequest { request: String } = "Invalid request: {request}",
    ParseJson { json: String } = "Json with non empty body: {json}"
}

pub type RequestResult<T, E = RequestErrors> = std::result::Result<T, E>;

custom_error! {pub ResponseErrors
    ParseJson = "Error parsing json."
}

pub type ResponseResult<T, E = ResponseErrors> = std::result::Result<T, E>;