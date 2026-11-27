use std::{fs, net::SocketAddr, sync::Arc};

use anyhow::Context;
use quinn::{
    ClientConfig, Endpoint,
    rustls::{RootCertStore, pki_types::CertificateDer},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cert_raw = fs::read("cert.der").context("failed to read cert.der")?;
    let cert = CertificateDer::from(cert_raw);

    let mut cert_store = RootCertStore::empty();
    cert_store.add(cert).context("failed to add certificate")?;

    let config = ClientConfig::with_root_certificates(Arc::new(cert_store))?;
    let server_addr: SocketAddr = "127.0.0.1:5000".parse()?;
    let local_addr: SocketAddr = "0.0.0.0:0".parse()?;

    let mut endpoint = Endpoint::client(local_addr)?;
    endpoint.set_default_client_config(config);

    let conn = endpoint
        .connect(server_addr, "localhost")?
        .await
        .context("handshake with server failed")?;
    println!("connected to {}", conn.remote_address());

    conn.close(0u32.into(), b"done");
    endpoint.wait_idle().await;
    Ok(())
}
