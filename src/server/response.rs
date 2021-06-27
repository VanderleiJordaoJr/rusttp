use crate::server::http_enums::ResponseStatusCode;
use serde_json::Value;
use std::collections::HashMap;
use crate::server_errors::{ResponseErrors, ResponseResult};
use serde_json::Result as serde_result;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Response {
    stream: TcpStream,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Response {stream}
    }

    pub fn send(&mut self, status_code: ResponseStatusCode) {
        self.send_all(status_code, None, None);
    }

    pub fn send_headers(&mut self, status_code: ResponseStatusCode, headers: Option<HashMap<String, String>>) {
        self.send_all(status_code, headers, None);
    }

    pub fn send_json(&mut self, status_code: ResponseStatusCode, json: Option<Value>) {
        self.send_all(status_code, None, json);
    }

    pub fn send_all(&mut self, status_code: ResponseStatusCode, headers: Option<HashMap<String, String>>, json: Option<Value>) {
        let parsed_string = Response::get_parsed_data(Response::parse_data(status_code, headers, json));

        self.stream.write(parsed_string.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    fn get_parsed_data(parsed_or_fail: ResponseResult<String, ResponseErrors>) -> String {
        match parsed_or_fail {
            Ok(value) => value,
            Err(_) => String::from("HTTP/1.1 500 INTERNAL SERVER ERROR\r\n")
        }
    }

    fn parse_data(status_code: ResponseStatusCode, headers: Option<HashMap<String, String>>, json: Option<Value>) -> ResponseResult<String, ResponseErrors> {
        let headers = match headers {
            Some(h) => format!("{}", Response::parse_headers(h)),
            None => String::from("")
        };

        let body: String = match json {
            Some(value) => {
                let serde_result: serde_result<String> = serde_json::to_string(&value);
                let text: String = match serde_result {
                    Ok(text) => text,
                    Err(_) => {
                        return Err(ResponseErrors::ParseJson);
                    }
                };

                let mut json_headers: HashMap<String, String> = HashMap::new();

                json_headers.insert(String::from("Content-Type"), String::from("application/json"));
                json_headers.insert(String::from("Content-Length"), text.as_bytes().len().to_string());

                format!("{}\r\n\r\n{}\r\n", Response::parse_headers(json_headers), text)
            }
            None => String::from("\r\n\r\n")
        };

        Ok(format!("HTTP/1.1 {}{}{}", status_code.to_string(), headers, body))
    }

    fn parse_headers(headers: HashMap<String, String>) -> String {
        let mut to_return = String::new();
        for (key, value) in headers.iter() {
            to_return.push_str(&format!("\r\n{}: {}", key, value)[..]);
        };
        to_return
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn send_without_body_without_header() {
        let to_get = Response::parse_data(ResponseStatusCode::OK, None, None);
        let result = Response::get_parsed_data(to_get);
        assert_eq!("HTTP/1.1 200 OK\r\n\r\n", result)
    }

    #[test]
    fn send_without_body_with_header() {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Accept"), String::from("*/*"));

        let to_get =
            Response::parse_data(ResponseStatusCode::NOT_FOUND,
                                 Some(headers), None);
        let result = Response::get_parsed_data(to_get);

        assert_eq!("HTTP/1.1 404 NOT FOUND\r\nAccept: */*\r\n\r\n", result)
    }

    #[test]
    fn send_with_body_without_header() {
        let json = serde_json::from_str("{\"id\":1,\"name\":\"Vand\",\"password\":\"123\"}").unwrap();

        let to_get =
            Response::parse_data(ResponseStatusCode::ACCEPTED,
                                 None ,Some(json) );
        let result = Response::get_parsed_data(to_get);
        let one = "HTTP/1.1 202 ACCEPTED\r\nContent-Type: application/json\r\nContent-Length: 39\r\n\r\n{\"id\":1,\"name\":\"Vand\",\"password\":\"123\"}\r\n";
        let two = "HTTP/1.1 202 ACCEPTED\r\nContent-Length: 39\r\nContent-Type: application/json\r\n\r\n{\"id\":1,\"name\":\"Vand\",\"password\":\"123\"}\r\n";
        assert!(one.eq(result.as_str()) || two.eq(result.as_str()))
    }
}