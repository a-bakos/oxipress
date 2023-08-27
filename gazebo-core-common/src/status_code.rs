// Usage:
// let status_code = HttpStatusCode::Unauthorized.code();
// let status_message = HttpStatusCode::Unauthorized.message();

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub enum HttpStatusCode {
    Continue,
    SwitchingProtocols,
    Processing,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl HttpStatusCode {
    #[allow(dead_code)]
    pub fn code(&self) -> u32 {
        match self {
            HttpStatusCode::Continue => 100,
            HttpStatusCode::SwitchingProtocols => 101,
            HttpStatusCode::Processing => 102,
            HttpStatusCode::Ok => 200,
            HttpStatusCode::Created => 201,
            HttpStatusCode::Accepted => 202,
            HttpStatusCode::NonAuthoritativeInformation => 203,
            HttpStatusCode::NoContent => 204,
            HttpStatusCode::ResetContent => 205,
            HttpStatusCode::PartialContent => 206,
            HttpStatusCode::MultiStatus => 207,
            HttpStatusCode::AlreadyReported => 208,
            HttpStatusCode::IMUsed => 226,
            HttpStatusCode::MultipleChoices => 300,
            HttpStatusCode::MovedPermanently => 301,
            HttpStatusCode::Found => 302,
            HttpStatusCode::SeeOther => 303,
            HttpStatusCode::NotModified => 304,
            HttpStatusCode::UseProxy => 305,
            HttpStatusCode::TemporaryRedirect => 307,
            HttpStatusCode::PermanentRedirect => 308,
            HttpStatusCode::BadRequest => 400,
            HttpStatusCode::Unauthorized => 401,
            HttpStatusCode::PaymentRequired => 402,
            HttpStatusCode::Forbidden => 403,
            HttpStatusCode::NotFound => 404,
            HttpStatusCode::MethodNotAllowed => 405,
            HttpStatusCode::NotAcceptable => 406,
            HttpStatusCode::ProxyAuthenticationRequired => 407,
            HttpStatusCode::RequestTimeout => 408,
            HttpStatusCode::Conflict => 409,
            HttpStatusCode::Gone => 410,
            HttpStatusCode::LengthRequired => 411,
            HttpStatusCode::PreconditionFailed => 412,
            HttpStatusCode::PayloadTooLarge => 413,
            HttpStatusCode::URITooLong => 414,
            HttpStatusCode::UnsupportedMediaType => 415,
            HttpStatusCode::RangeNotSatisfiable => 416,
            HttpStatusCode::ExpectationFailed => 417,
            HttpStatusCode::ImATeapot => 418,
            HttpStatusCode::MisdirectedRequest => 421,
            HttpStatusCode::UnprocessableEntity => 422,
            HttpStatusCode::Locked => 423,
            HttpStatusCode::FailedDependency => 424,
            HttpStatusCode::TooEarly => 425,
            HttpStatusCode::UpgradeRequired => 426,
            HttpStatusCode::PreconditionRequired => 428,
            HttpStatusCode::TooManyRequests => 429,
            HttpStatusCode::RequestHeaderFieldsTooLarge => 431,
            HttpStatusCode::UnavailableForLegalReasons => 451,
            HttpStatusCode::InternalServerError => 500,
            HttpStatusCode::NotImplemented => 501,
            HttpStatusCode::BadGateway => 502,
            HttpStatusCode::ServiceUnavailable => 503,
            HttpStatusCode::GatewayTimeout => 504,
            HttpStatusCode::HTTPVersionNotSupported => 505,
            HttpStatusCode::VariantAlsoNegotiates => 506,
            HttpStatusCode::InsufficientStorage => 507,
            HttpStatusCode::LoopDetected => 508,
            HttpStatusCode::NotExtended => 510,
            HttpStatusCode::NetworkAuthenticationRequired => 511,
        }
    }

    #[allow(dead_code)]
    pub fn message(&self) -> &str {
        match self {
            HttpStatusCode::Continue => "Continue",
            HttpStatusCode::SwitchingProtocols => "Switching Protocols",
            HttpStatusCode::Processing => "Processing",
            HttpStatusCode::Ok => "OK",
            HttpStatusCode::Created => "Created",
            HttpStatusCode::Accepted => "Accepted",
            HttpStatusCode::NonAuthoritativeInformation => "Non-Authoritative Information",
            HttpStatusCode::NoContent => "No Content",
            HttpStatusCode::ResetContent => "Reset Content",
            HttpStatusCode::PartialContent => "Partial Content",
            HttpStatusCode::MultiStatus => "Multi-Status",
            HttpStatusCode::AlreadyReported => "Already Reported",
            HttpStatusCode::IMUsed => "IM Used",
            HttpStatusCode::MultipleChoices => "Multiple Choices",
            HttpStatusCode::MovedPermanently => "Moved Permanently",
            HttpStatusCode::Found => "Found",
            HttpStatusCode::SeeOther => "See Other",
            HttpStatusCode::NotModified => "Not Modified",
            HttpStatusCode::UseProxy => "Use Proxy",
            HttpStatusCode::TemporaryRedirect => "Temporary Redirect",
            HttpStatusCode::PermanentRedirect => "Permanent Redirect",
            HttpStatusCode::BadRequest => "Bad Request",
            HttpStatusCode::Unauthorized => "Unauthorized",
            HttpStatusCode::PaymentRequired => "Payment Required",
            HttpStatusCode::Forbidden => "Forbidden",
            HttpStatusCode::NotFound => "Not Found",
            HttpStatusCode::MethodNotAllowed => "Method Not Allowed",
            HttpStatusCode::NotAcceptable => "Not Acceptable",
            HttpStatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpStatusCode::RequestTimeout => "Request Timeout",
            HttpStatusCode::Conflict => "Conflict",
            HttpStatusCode::Gone => "Gone",
            HttpStatusCode::LengthRequired => "Length Required",
            HttpStatusCode::PreconditionFailed => "Precondition Failed",
            HttpStatusCode::PayloadTooLarge => "Payload Too Large",
            HttpStatusCode::URITooLong => "URI Too Long",
            HttpStatusCode::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatusCode::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatusCode::ExpectationFailed => "Expectation Failed",
            HttpStatusCode::ImATeapot => "I'm a teapot",
            HttpStatusCode::MisdirectedRequest => "Misdirected Request",
            HttpStatusCode::UnprocessableEntity => "Unprocessable Entity",
            HttpStatusCode::Locked => "Locked",
            HttpStatusCode::FailedDependency => "Failed Dependency",
            HttpStatusCode::TooEarly => "Too Early",
            HttpStatusCode::UpgradeRequired => "Upgrade Required",
            HttpStatusCode::PreconditionRequired => "Precondition Required",
            HttpStatusCode::TooManyRequests => "Too Many Requests",
            HttpStatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpStatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            HttpStatusCode::InternalServerError => "Internal Server Error",
            HttpStatusCode::NotImplemented => "Not Implemented",
            HttpStatusCode::BadGateway => "Bad Gateway",
            HttpStatusCode::ServiceUnavailable => "Service Unavailable",
            HttpStatusCode::GatewayTimeout => "Gateway Timeout",
            HttpStatusCode::HTTPVersionNotSupported => "HTTP Version Not Supported",
            HttpStatusCode::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpStatusCode::InsufficientStorage => "Insufficient Storage",
            HttpStatusCode::LoopDetected => "Loop Detected",
            HttpStatusCode::NotExtended => "Not Extended",
            HttpStatusCode::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }
}
