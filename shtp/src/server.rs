use crate::{abc::DeviceType};
use crate::handler::{SHTPHandler, SHTPRequest};
use std::{io, net};

pub struct SHTPServer<T> {
    server: net::TcpListener,
    device_type: DeviceType,
    handler: T,
}

impl<T: SHTPHandler> SHTPServer<T> {
    pub fn new(host: &str, port: &u16, device_type: DeviceType, handler: T) -> io::Result<Self> {
        let server = net::TcpListener::bind(format!("{}:{}", host, port))?;

        Ok(Self {
            server,
            device_type,
            handler,
        })
    }

    pub fn listen(&self) {
        for connection in self.server.incoming() {
            match connection {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(error) => {
                    print!("Network error: {}", error);
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: net::TcpStream) {
        match SHTPRequest::receive(&mut stream) {
            Ok(request) => {
                if request.device_type != self.device_type {
                    print!("Bad device type in request");
                    return ;
                }

                self.handler.on_request(&request);
            },
            Err(error) => {
                println!("Error on handling request: {}", error)
            }
        }
    }
}
