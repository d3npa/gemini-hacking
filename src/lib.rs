use std::error::Error;
use tokio_native_tls::native_tls;
use tokio_native_tls::TlsAcceptor;
use tokio::net::TcpListener;

pub use url;
pub use urlencoding as ue;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub const MAX_HEADER_LENGTH: usize = 1026;

pub mod response_codes {
    pub const INPUT                         : usize = 10;
    pub const SENSITIVE_INPUT               : usize = 11;

    pub const SUCCESS                       : usize = 20;

    pub const REDIRECT_TEMPORARY            : usize = 30;
    pub const REDIRECT_PERMANENT            : usize = 31;

    pub const TEMPORARY_FAILURE             : usize = 40;
    pub const SERVER_UNAVAILABLE            : usize = 41;
    pub const CGI_ERROR                     : usize = 42;
    pub const PROXY_ERROR                   : usize = 43;
    pub const SLOW_DOWN                     : usize = 44;

    pub const PERMANENT_FAILURE             : usize = 50;
    pub const NOT_FOUND                     : usize = 51;
    pub const GONE                          : usize = 52;
    pub const PROXY_REQUEST_REFUSED         : usize = 53;
    pub const BAD_REQUEST                   : usize = 59;

    pub const CLIENT_CERTIFICATE_REQUIRED   : usize = 60;
    pub const CERTIFICATE_NOT_AUTHORIZED    : usize = 61;
    pub const CERTIFICATE_NOT_VALID         : usize = 62;
}

const GEMINI_LOCALHOST: &str = "127.0.0.1:1965";
const X509_PROFILE: &[u8] = include_bytes!("../identity.pfx");

pub fn create_tls_acceptor() -> Result<TlsAcceptor> {
    let identity = native_tls::Identity::from_pkcs12(&X509_PROFILE, "")?;
    let acceptor = native_tls::TlsAcceptor::new(identity)?;
    Ok(TlsAcceptor::from(acceptor))
}

pub async fn create_tcp_listener() -> Result<TcpListener> {
    let listener = TcpListener::bind(&GEMINI_LOCALHOST).await?;
    Ok(listener)
}

pub fn create_response(
    code: usize, meta: Option<&str>, body: Option<&str>) -> Vec<u8>
{
    let meta = meta.unwrap_or("");
    let mut response = format!("{} {}\r\n", code, meta);

    if let Some(body) = body {
        response.push_str(body)
    }

    response.into_bytes()
}