extern crate custom_error;

use custom_error::custom_error;

custom_error! {pub ServerErrors
    HTTPHeader { request: String } = "Invalid request header: {request}",
    HTTPRequest { method: String } = "Invalid method: {method}",
    UnparsedRequest { request: String } = "Invalid request: {request}",
    ParseJson { json: String } = "Json with non empty body: {json}"
}

pub type Result<T, E = ServerErrors> = std::result::Result<T, E>;