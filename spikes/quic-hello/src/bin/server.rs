use std::{fs, net::SocketAddr};

use anyhow::Context;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rcgen::CertifiedKey { cert, signing_key } =
        rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;
    let cert_der = CertificateDer::from(cert);
    let key = PrivateKeyDer::from(signing_key);
    fs::write("cert.der", &cert_der)?;
    let config = quinn::ServerConfig::with_single_cert(vec![cert_der], key)?;
    let socket_addr = SocketAddr::V4("127.0.0.1:5000".parse()?);
    let endpoint =
        quinn::Endpoint::server(config, socket_addr).context("failed to bind 127.0.0.1:5000")?;
    println!("listening");
    let incoming = endpoint
        .accept()
        .await
        .context("endpoint closed before any connection")?;
    let conn = incoming.await?;
    println!("{}", conn.remote_address());
    Ok(())
}
