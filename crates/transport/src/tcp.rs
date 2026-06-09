use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub async fn connect(addr: SocketAddr) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(addr).await
}

pub async fn listen(addr: SocketAddr) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(addr).await
}
