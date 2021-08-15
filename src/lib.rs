use std::error::Error;
use tokio_native_tls::native_tls;
use tokio_native_tls::TlsAcceptor;
use tokio::net::TcpListener;

type Result<T> = std::result::Result<T, Box<dyn Error>>;


pub const MAX_HEADER_LENGTH: usize = 1026;

pub mod response_codes {
    pub const INPUT: usize = 10;
    pub const INPUT_SENSITIVE: usize = 11;
    pub const SUCCESS: usize = 20;
    pub const REDIRECT_TEMPORARY: usize = 30;
    pub const REDIRECT_PERMANENT: usize = 31;
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