use std::fmt;

#[derive(Debug,PartialEq)]
pub enum RequestMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}

impl RequestMethod {
    pub fn from_str(method: &str) -> Option<RequestMethod> {
        match method {
            "CONNECT" => Some(RequestMethod::CONNECT),
            "DELETE" => Some(RequestMethod::DELETE),
            "GET" => Some(RequestMethod::GET),
            "HEAD" => Some(RequestMethod::HEAD),
            "OPTIONS" => Some(RequestMethod::OPTIONS),
            "PATCH" => Some(RequestMethod::PATCH),
            "POST" => Some(RequestMethod::POST),
            "PUT" => Some(RequestMethod::PUT),
            "TRACE" => Some(RequestMethod::TRACE),
            _ => None
        }
    }
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestMethod::CONNECT => f.write_str("CONNECT"),
            RequestMethod::DELETE => f.write_str("DELETE"),
            RequestMethod::GET => f.write_str("GET"),
            RequestMethod::HEAD => f.write_str("HEAD"),
            RequestMethod::OPTIONS => f.write_str("OPTIONS"),
            RequestMethod::PATCH => f.write_str("PATCH"),
            RequestMethod::POST => f.write_str("POST"),
            RequestMethod::PUT => f.write_str("PUT"),
            RequestMethod::TRACE => f.write_str("TRACE")
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug,PartialEq)]
pub enum ResponseStatusCode {
    // 10x - Information responses
    CONTINUE,
    SWITCHING_PROTOCOLS,
    EARLY_HINTS,

    // 20X - Success responses
    OK,
    CREATED,
    ACCEPTED,
    NON_AUTHORITATIVE_INFORMATION,
    NO_CONTENT,
    RESET_CONTENT,
    PARTIAL_CONTENT,

    // 30X - Redirects
    MULTIPLE_CHOICES,
    MOVED_PERMANENTLY,
    FOUND,
    SEE_OTHER,
    NOT_MODIFIED,
    TEMPORARY_REDIRECT,
    PERMANENT_REDIRECT,

    // 40X - Client errors
    BAD_REQUEST,
    UNAUTHORIZED,
    PAYMENT_REQUIRED,
    FORBIDDEN,
    NOT_FOUND,
    METHOD_NOT_ALLOWED,
    NOT_ACCEPTABLE,
    PROXY_AUTHENTICATION_REQUIRED,
    REQUEST_TIMEOUT,
    CONFLICT,
    GONE,
    LENGTH_REQUIRED,
    PRECONDITION_FAILED,
    PAYLOAD_TOO_LARGE,
    URI_TOO_LONG,
    UNSUPPORTED_MEDIA_TYPE,
    RANGE_NOT_SATISFIABLE,
    EXPECTATION_FAILED,
    IM_A_TEAPOT,
    UNPROCESSABLE_ENTITY,
    TOO_EARLY,
    UPGRADE_REQUIRED,
    PRECONDITION_REQUIRED,
    TOO_MANY_REQUESTS,
    REQUEST_HEADER_FIELDS_TOO_LARGE,
    UNAVAILABLE_FOR_LEGAL_REASONS,

    // 50X - Server errors
    INTERNAL_SERVER_ERROR,
    NOT_IMPLEMENTED,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,
    GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED,
    VARIANT_ALSO_NEGOTIATES,
    INSUFFICIENT_STORAGE,
    LOOP_DETECTED,
    NOT_EXTENDED,
    NETWORK_AUTHENTICATION_REQUIRED,
}

impl fmt::Display for ResponseStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // 10x - Information responses
            ResponseStatusCode::CONTINUE => f.write_str("100 CONTINUE"),
            ResponseStatusCode::SWITCHING_PROTOCOLS => f.write_str("101 SWITCHING PROTOCOLS"),
            ResponseStatusCode::EARLY_HINTS => f.write_str("102 EARLY HINTS"),

            // 20X - Success responses
            ResponseStatusCode::OK => f.write_str("200 OK"),
            ResponseStatusCode::CREATED => f.write_str("201 CREATED"),
            ResponseStatusCode::ACCEPTED => f.write_str("202 ACCEPTED"),
            ResponseStatusCode::NON_AUTHORITATIVE_INFORMATION => f.write_str("203 NON AUTHORITATIVE INFORMATION"),
            ResponseStatusCode::NO_CONTENT => f.write_str("204 NO CONTENT"),
            ResponseStatusCode::RESET_CONTENT => f.write_str("205 RESET CONTENT"),
            ResponseStatusCode::PARTIAL_CONTENT => f.write_str("206 PARTIAL CONTENT"),

            // 30X - Redirects
            ResponseStatusCode::MULTIPLE_CHOICES => f.write_str("300 MULTIPLE CHOICES"),
            ResponseStatusCode::MOVED_PERMANENTLY => f.write_str("301 MOVED PERMANENTLY"),
            ResponseStatusCode::FOUND => f.write_str("302 FOUND"),
            ResponseStatusCode::SEE_OTHER => f.write_str("303 SEE OTHER"),
            ResponseStatusCode::NOT_MODIFIED => f.write_str("304 NOT MODIFIED"),
            ResponseStatusCode::TEMPORARY_REDIRECT => f.write_str("305 TEMPORARY REDIRECT"),
            ResponseStatusCode::PERMANENT_REDIRECT => f.write_str("306 PERMANENT REDIRECT"),

            // 40X - Client errors
            ResponseStatusCode::BAD_REQUEST => f.write_str("400 BAD REQUEST"),
            ResponseStatusCode::UNAUTHORIZED => f.write_str("401 UNAUTHORIZED"),
            ResponseStatusCode::PAYMENT_REQUIRED => f.write_str("402 PAYMENT REQUIRED"),
            ResponseStatusCode::FORBIDDEN => f.write_str("403 FORBIDDEN"),
            ResponseStatusCode::NOT_FOUND => f.write_str("404 NOT FOUND"),
            ResponseStatusCode::METHOD_NOT_ALLOWED => f.write_str("405 METHOD NOT ALLOWED"),
            ResponseStatusCode::NOT_ACCEPTABLE => f.write_str("406 NOT ACCEPTABLE"),
            ResponseStatusCode::PROXY_AUTHENTICATION_REQUIRED => f.write_str("407 PROXY AUTHENTICATION REQUIRED"),
            ResponseStatusCode::REQUEST_TIMEOUT => f.write_str("408 REQUEST TIMEOUT"),
            ResponseStatusCode::CONFLICT => f.write_str("409 CONFLICT"),
            ResponseStatusCode::GONE => f.write_str("410 GONE"),
            ResponseStatusCode::LENGTH_REQUIRED => f.write_str("411 LENGTH REQUIRED"),
            ResponseStatusCode::PRECONDITION_FAILED => f.write_str("412 PRECONDITION FAILED"),
            ResponseStatusCode::PAYLOAD_TOO_LARGE => f.write_str("413 PAYLOAD TOO LARGE"),
            ResponseStatusCode::URI_TOO_LONG => f.write_str("414 URI TOO LONG"),
            ResponseStatusCode::UNSUPPORTED_MEDIA_TYPE => f.write_str("415 UNSUPPORTED MEDIA TYPE"),
            ResponseStatusCode::RANGE_NOT_SATISFIABLE => f.write_str("416 RANGE NOT SATISFIABLE"),
            ResponseStatusCode::EXPECTATION_FAILED => f.write_str("417 EXPECTATION FAILED"),
            ResponseStatusCode::IM_A_TEAPOT => f.write_str("418 IM A_TEAPOT"),
            ResponseStatusCode::UNPROCESSABLE_ENTITY => f.write_str("422 UNPROCESSABLE ENTITY"),
            ResponseStatusCode::TOO_EARLY => f.write_str("425 TOO EARLY"),
            ResponseStatusCode::UPGRADE_REQUIRED => f.write_str("426 UPGRADE REQUIRED"),
            ResponseStatusCode::PRECONDITION_REQUIRED => f.write_str("428 PRECONDITION REQUIRED"),
            ResponseStatusCode::TOO_MANY_REQUESTS => f.write_str("429 TOO MANY_REQUESTS"),
            ResponseStatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => f.write_str("431 REQUEST HEADER FIELDS TOO LARGE"),
            ResponseStatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => f.write_str("451 UNAVAILABLE FOR LEGAL REASONS"),

            // 50X - Server errors
            ResponseStatusCode::INTERNAL_SERVER_ERROR => f.write_str("500 INTERNAL SERVER ERROR"),
            ResponseStatusCode::NOT_IMPLEMENTED => f.write_str("501 NOT IMPLEMENTED"),
            ResponseStatusCode::BAD_GATEWAY => f.write_str("502 BAD GATEWAY"),
            ResponseStatusCode::SERVICE_UNAVAILABLE => f.write_str("503 SERVICE UNAVAILABLE"),
            ResponseStatusCode::GATEWAY_TIMEOUT => f.write_str("504 GATEWAY TIMEOUT"),
            ResponseStatusCode::HTTP_VERSION_NOT_SUPPORTED => f.write_str("505 HTTP VERSION NOT SUPPORTED"),
            ResponseStatusCode::VARIANT_ALSO_NEGOTIATES => f.write_str("506 VARIANT ALSO NEGOTIATES"),
            ResponseStatusCode::INSUFFICIENT_STORAGE => f.write_str("507 INSUFFICIENT STORAGE"),
            ResponseStatusCode::LOOP_DETECTED => f.write_str("508 LOOP DETECTED"),
            ResponseStatusCode::NOT_EXTENDED => f.write_str("510 NOT EXTENDED"),
            ResponseStatusCode::NETWORK_AUTHENTICATION_REQUIRED => f.write_str("511 NETWORK AUTHENTICATION REQUIRED")
        }
    }
}
