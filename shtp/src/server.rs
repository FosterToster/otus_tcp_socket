use crate::device_type::DeviceType;
use crate::handler::{SHTPHandler, SHTPRequest, SHTPResponse};
use std::{io, io::Write, net};

pub struct SHTPServer<T> {
    server: net::TcpListener,
    device_type: DeviceType,
    handler: T,
}

impl<T: SHTPHandler> SHTPServer<T> {
    pub fn new(host: String, port: u16, device_type: DeviceType, handler: T) -> io::Result<Self> {
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
                    self.respond(
                        &mut stream,
                        SHTPResponse::fail("Bad device type in request"),
                    );
                }

                self.respond(&mut stream, self.handler.on_request(&request));
            }
            Err(error) => {
                self.respond(&mut stream, SHTPResponse::fail(&format!("{}", error)));
                println!("Error on handling request: {}", error);
            }
        }
    }

    fn respond<S: Write>(&self, stream: &mut S, response: SHTPResponse) {
        if let Err(error) = response.send(stream) {
            println!("Failed to respond: {}", error);
        }
    }
}
