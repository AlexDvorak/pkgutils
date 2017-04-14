use std::fs::File;
use std::io::{self, stderr, Write};
use std::error::Error;
use std::time::Duration;

use hyper::status::StatusCode;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;
use hyper::error::Error as HyperError;

pub fn download(remote_path: &str, local_path: &str) -> io::Result<()> {
    write!(stderr(), "* Requesting {}\n", remote_path)?;

    let mut client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
    client.set_read_timeout(Some(Duration::new(5, 0)));
    client.set_write_timeout(Some(Duration::new(5, 0)));
    let mut res = match client.get(remote_path).send() {
        Ok(res) => res,
        Err(HyperError::Io(err)) => return Err(err),
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.description()))
    };

    match res.status {
        StatusCode::Ok => {
            write!(stderr(), "* Success {}\n", res.status)?;

            let mut file = File::create(&local_path)?;
            io::copy(&mut res, &mut file)?;

            Ok(())
        },
        _ => {
            write!(stderr(), "* Failure {}\n", res.status)?;

            Err(io::Error::new(io::ErrorKind::NotFound, format!("{} not found", remote_path)))
        }
    }
}