use crate::http_enums::RequestMethod;
use std::collections::HashMap;
use serde_json::Value;
use crate::server_errors::{Result, ServerErrors};
use std::fmt;

#[derive(Debug)]
pub struct Request {
    method: RequestMethod,
    path: String,
    headers: HashMap<String, String>,
    body: Option<Value>,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method: {}; path: {}, headers: {:?}, body: {:?}", self.method,
               self.path, self.headers, self.body)
    }
}

impl Request {
    pub fn from_str(request_txt: &str) -> Result<Request, ServerErrors> {
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut header_result: Result<(RequestMethod, String), ServerErrors> = {
            Err(ServerErrors::UnparsedRequest { request: "".to_string() })
        };

        let (raw_headers, raw_body) = match separate_body_from_header(request_txt) {
            Ok(separated) => (separated.0, separated.1),
            Err(e) => return Err(e)
        };

        for line in raw_headers.lines() {
            if line.contains("HTTP/1.1") {
                header_result = parse_request_header(line);
            } else if line.contains(":") {
                let separated_header: Vec<&str> = line.split(": ").collect();
                headers.insert(separated_header[0].parse().unwrap(), separated_header[1].parse().unwrap());
            }
        }

        let body_parse_result: Result<Option<Value>, ServerErrors> = match raw_body {
            "" => Ok(None),
            json => {
                match serde_json::from_str(json) {
                    Ok(value) => Ok(Some(value)),
                    Err(_) => {
                        Err(ServerErrors::ParseJson { json: String::from(json) })
                    }
                }
            }
        };

        let (method, path) = match header_result {
            Ok(result) => {
                (result.0, result.1)
            }
            Err(err) => {
                return Err(err);
            }
        };

        let body: Option<Value> = match body_parse_result {
            Ok(result) => {
                result
            }
            Err(err) => {
                return Err(err);
            }
        };

        Ok(Request {
            method,
            path,
            headers,
            body,
        })
    }
}

fn parse_request_header(header_line: &str) -> Result<(RequestMethod, String), ServerErrors> {
    let separated_header: Vec<&str> = header_line.split(" ").collect();

    let method_str: &str = match separated_header.get(0) {
        Some(path) => {
            path
        }
        None => {
            return Err(ServerErrors::HTTPHeader { request: String::from(header_line) });
        }
    };

    let path: &str = match separated_header.get(1) {
        Some(path) => {
            path
        }
        None => {
            return Err(ServerErrors::HTTPHeader { request: String::from(header_line) });
        }
    };

    let request_method = match RequestMethod::from_str(method_str) {
        Some(method) => method,
        None => {
            return Err(ServerErrors::HTTPRequest { method: String::from(method_str) });
        }
    };

    Ok((request_method, String::from(path)))
}

fn separate_body_from_header(request_txt: &str) -> Result<(&str, &str), ServerErrors> {
    let separated_vec: Vec<&str> = request_txt.split("\r\n\r\n").collect();
    let request_header = separated_vec[0];

    let request_body = match separated_vec.get(1) {
        Some(body) => body,
        None => return Err(ServerErrors::UnparsedRequest { request: String::from(request_txt) })
    };

    Ok((request_header, request_body))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_request_ok() {
        let to_parse = "GET / HTTP/1.1\r\nHost: localhost:8378\r\nUser-Agent: insomnia/2021.3.0\r\nAccept: */*\r\n\r\n";
        let request = match Request::from_str(to_parse) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("An error occurred: {:?}", e);
                panic!("Panicing");
            }
        };
        assert_eq!(request.method, RequestMethod::GET);
        assert_eq!(request.path, "/");
        assert_eq!(request.headers.len(), 3);
    }

    #[test]
    fn parse_request_nothing() {
        let to_parse = "\r\n";
        let error = match Request::from_str(to_parse) {
            Ok(req) => {
                println!("An error occurred: {}", req);
                panic!("{}", req)
            }
            Err(e) => e
        };
        assert_eq!(error, ServerErrors::UnparsedRequest{request: String::from(to_parse)})
    }

    #[test]
    fn invalid_method() {
        let to_parse = "INVALID / HTTP/1.1\r\nHost: localhost:8378\r\nUser-Agent: insomnia/2021.3.0\r\nAccept: */*\r\n\r\n";
        let error = match Request::from_str(to_parse) {
            Ok(req) => {
                println!("An error occurred: {}", req);
                panic!("{}", req)
            }
            Err(e) => e
        };
        assert_eq!(error, ServerErrors::HTTPRequest { method: String::from("INVALID") })
    }

    #[test]
    fn invalid_json() {
        let to_parse = "POST / HTTP/1.1\r\nHost: localhost:8378\r\nUser-Agent: insomnia/2021.3.0\r\nAccept: */*\r\n\r\nhey";
        let error = match Request::from_str(to_parse) {
            Ok(req) => {
                println!("An error occurred: {}", req);
                panic!("{}", req)
            }
            Err(e) => e
        };
        assert_eq!(error, ServerErrors::ParseJson { json: String::from("hey") })
    }
}

/*/*
GET /test HTTP/1.1
Host: localhost:8378
User-Agent: insomnia/2021.3.0
Accept: */
*/