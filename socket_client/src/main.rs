use std::io::{self, Write};

mod client;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 6411;

fn main() {
    let mut client = client::SHTPElectricSocketClient::new(HOST.to_string(), PORT);

    println!("Welcome to electric socket client!");
    println!("Welcome to electric socket client!");
    println!("Type any command you want to see result");
    println!("Or type 'exit!' instead to exit");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command == "exit!\r\n" {
            break;
        }

        match client.send_any_command(command.strip_suffix("\r\n").unwrap()) {
            Ok(response) => match response.observe() {
                Ok(result) => {
                    println!("<< done: {}", result)
                }
                Err(message) => {
                    println!("<< fail: {}", message)
                }
            },
            Err(error) => {
                println!("Unable to send request: {}", error)
            }
        }
    }
}
