use std::fmt;

pub enum RequestMethods {
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

impl RequestMethods {
    pub fn from_str(method: &str) -> Option<RequestMethods> {
        match method {
            "CONNECT" => Some(RequestMethods::CONNECT),
            "DELETE" => Some(RequestMethods::DELETE),
            "GET" => Some(RequestMethods::GET),
            "HEAD" => Some(RequestMethods::HEAD),
            "OPTIONS" => Some(RequestMethods::OPTIONS),
            "PATCH" => Some(RequestMethods::PATCH),
            "POST" => Some(RequestMethods::POST),
            "PUT" => Some(RequestMethods::PUT),
            "TRACE" => Some(RequestMethods::TRACE),
            _ => None
        }
    }
}

impl fmt::Display for RequestMethods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestMethods::CONNECT => f.write_str("CONNECT"),
            RequestMethods::DELETE => f.write_str("DELETE"),
            RequestMethods::GET => f.write_str("GET"),
            RequestMethods::HEAD => f.write_str("HEAD"),
            RequestMethods::OPTIONS => f.write_str("OPTIONS"),
            RequestMethods::PATCH => f.write_str("PATCH"),
            RequestMethods::POST => f.write_str("POST"),
            RequestMethods::PUT => f.write_str("PUT"),
            RequestMethods::TRACE => f.write_str("TRACE")
        }
    }
}

pub enum ResponseStatusCodes {
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

impl fmt::Display for ResponseStatusCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // 10x - Information responses
            ResponseStatusCodes::CONTINUE => f.write_str("100 CONTINUE"),
            ResponseStatusCodes::SWITCHING_PROTOCOLS => f.write_str("101 SWITCHING PROTOCOLS"),
            ResponseStatusCodes::EARLY_HINTS => f.write_str("102 EARLY HINTS"),

            // 20X - Success responses
            ResponseStatusCodes::OK => f.write_str("200 OK"),
            ResponseStatusCodes::CREATED => f.write_str("201 CREATED"),
            ResponseStatusCodes::ACCEPTED => f.write_str("202 ACCEPTED"),
            ResponseStatusCodes::NON_AUTHORITATIVE_INFORMATION => f.write_str("203 NON AUTHORITATIVE INFORMATION"),
            ResponseStatusCodes::NO_CONTENT => f.write_str("204 NO CONTENT"),
            ResponseStatusCodes::RESET_CONTENT => f.write_str("205 RESET CONTENT"),
            ResponseStatusCodes::PARTIAL_CONTENT => f.write_str("206 PARTIAL CONTENT"),

            // 30X - Redirects
            ResponseStatusCodes::MULTIPLE_CHOICES => f.write_str("300 MULTIPLE CHOICES"),
            ResponseStatusCodes::MOVED_PERMANENTLY => f.write_str("301 MOVED PERMANENTLY"),
            ResponseStatusCodes::FOUND => f.write_str("302 FOUND"),
            ResponseStatusCodes::SEE_OTHER => f.write_str("303 SEE OTHER"),
            ResponseStatusCodes::NOT_MODIFIED => f.write_str("304 NOT MODIFIED"),
            ResponseStatusCodes::TEMPORARY_REDIRECT => f.write_str("305 TEMPORARY REDIRECT"),
            ResponseStatusCodes::PERMANENT_REDIRECT => f.write_str("306 PERMANENT REDIRECT"),

            // 40X - Client errors
            ResponseStatusCodes::BAD_REQUEST => f.write_str("400 BAD REQUEST"),
            ResponseStatusCodes::UNAUTHORIZED => f.write_str("401 UNAUTHORIZED"),
            ResponseStatusCodes::PAYMENT_REQUIRED => f.write_str("402 PAYMENT REQUIRED"),
            ResponseStatusCodes::FORBIDDEN => f.write_str("403 FORBIDDEN"),
            ResponseStatusCodes::NOT_FOUND => f.write_str("404 NOT FOUND"),
            ResponseStatusCodes::METHOD_NOT_ALLOWED => f.write_str("405 METHOD NOT ALLOWED"),
            ResponseStatusCodes::NOT_ACCEPTABLE => f.write_str("406 NOT ACCEPTABLE"),
            ResponseStatusCodes::PROXY_AUTHENTICATION_REQUIRED => f.write_str("407 PROXY AUTHENTICATION REQUIRED"),
            ResponseStatusCodes::REQUEST_TIMEOUT => f.write_str("408 REQUEST TIMEOUT"),
            ResponseStatusCodes::CONFLICT => f.write_str("409 CONFLICT"),
            ResponseStatusCodes::GONE => f.write_str("410 GONE"),
            ResponseStatusCodes::LENGTH_REQUIRED => f.write_str("411 LENGTH REQUIRED"),
            ResponseStatusCodes::PRECONDITION_FAILED => f.write_str("412 PRECONDITION FAILED"),
            ResponseStatusCodes::PAYLOAD_TOO_LARGE => f.write_str("413 PAYLOAD TOO LARGE"),
            ResponseStatusCodes::URI_TOO_LONG => f.write_str("414 URI TOO LONG"),
            ResponseStatusCodes::UNSUPPORTED_MEDIA_TYPE => f.write_str("415 UNSUPPORTED MEDIA TYPE"),
            ResponseStatusCodes::RANGE_NOT_SATISFIABLE => f.write_str("416 RANGE NOT SATISFIABLE"),
            ResponseStatusCodes::EXPECTATION_FAILED => f.write_str("417 EXPECTATION FAILED"),
            ResponseStatusCodes::IM_A_TEAPOT => f.write_str("418 IM A_TEAPOT"),
            ResponseStatusCodes::UNPROCESSABLE_ENTITY => f.write_str("422 UNPROCESSABLE ENTITY"),
            ResponseStatusCodes::TOO_EARLY => f.write_str("425 TOO EARLY"),
            ResponseStatusCodes::UPGRADE_REQUIRED => f.write_str("426 UPGRADE REQUIRED"),
            ResponseStatusCodes::PRECONDITION_REQUIRED => f.write_str("428 PRECONDITION REQUIRED"),
            ResponseStatusCodes::TOO_MANY_REQUESTS => f.write_str("429 TOO MANY_REQUESTS"),
            ResponseStatusCodes::REQUEST_HEADER_FIELDS_TOO_LARGE => f.write_str("431 REQUEST HEADER FIELDS TOO LARGE"),
            ResponseStatusCodes::UNAVAILABLE_FOR_LEGAL_REASONS => f.write_str("451 UNAVAILABLE FOR LEGAL REASONS"),

            // 50X - Server errors
            ResponseStatusCodes::INTERNAL_SERVER_ERROR => f.write_str("500 INTERNAL SERVER ERROR"),
            ResponseStatusCodes::NOT_IMPLEMENTED => f.write_str("501 NOT IMPLEMENTED"),
            ResponseStatusCodes::BAD_GATEWAY => f.write_str("502 BAD GATEWAY"),
            ResponseStatusCodes::SERVICE_UNAVAILABLE => f.write_str("503 SERVICE UNAVAILABLE"),
            ResponseStatusCodes::GATEWAY_TIMEOUT => f.write_str("504 GATEWAY TIMEOUT"),
            ResponseStatusCodes::HTTP_VERSION_NOT_SUPPORTED => f.write_str("505 HTTP VERSION NOT SUPPORTED"),
            ResponseStatusCodes::VARIANT_ALSO_NEGOTIATES => f.write_str("506 VARIANT ALSO NEGOTIATES"),
            ResponseStatusCodes::INSUFFICIENT_STORAGE => f.write_str("507 INSUFFICIENT STORAGE"),
            ResponseStatusCodes::LOOP_DETECTED => f.write_str("508 LOOP DETECTED"),
            ResponseStatusCodes::NOT_EXTENDED => f.write_str("510 NOT EXTENDED"),
            ResponseStatusCodes::NETWORK_AUTHENTICATION_REQUIRED => f.write_str("511 NETWORK AUTHENTICATION REQUIRED")
        }
    }
}
