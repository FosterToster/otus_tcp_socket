use crate::device_type::DeviceType;
use crate::handler::{SHTPHandler, SHTPRequest, SHTPResponse};

pub struct SHTPServer<T> {
    server: tokio::net::TcpListener,
    device_type: DeviceType,
    handler: T,
}

impl<T: SHTPHandler> SHTPServer<T> {
    pub async fn new(
        host: String,
        port: u16,
        device_type: DeviceType,
        handler: T,
    ) -> std::io::Result<Self> {
        let server = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;

        Ok(Self {
            server,
            device_type,
            handler,
        })
    }

    pub async fn listen(&self) {
        loop {
            match self.server.accept().await {
                Ok((stream, _)) => {
                    self.handle_connection(stream).await;
                }
                Err(error) => {
                    println!("Network error: {}", error);
                }
            }
        }
    }

    async fn handle_connection(&self, mut stream: tokio::net::TcpStream) {
        match SHTPRequest::receive(&mut stream).await {
            Ok(request) => {
                if request.device_type != self.device_type {
                    self.respond(
                        &mut stream,
                        SHTPResponse::fail("Bad device type in request").await,
                    )
                    .await;
                    return;
                }

                self.respond(&mut stream, self.handler.on_request(&request).await)
                    .await;
            }
            Err(error) => {
                self.respond(&mut stream, SHTPResponse::fail(&format!("{}", error)).await)
                    .await;
                println!("Error on handling request: {}", error);
            }
        }
    }

    async fn respond<S: tokio::io::AsyncWrite + std::marker::Unpin>(
        &self,
        stream: &mut S,
        response: SHTPResponse,
    ) {
        if let Err(error) = response.send(stream).await {
            println!("Failed to respond: {}", error);
        }
    }
}
