use socket_client::{client, HOST, PORT};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let mut client = client::SHTPElectricSocketClient::new(HOST.to_string(), PORT).await;

    println!("Welcome to electric socket client!");
    println!("=======================================");
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

        let mut split = command.strip_suffix("\r\n").unwrap().split_whitespace();

        let command = split.next().unwrap();
        let args = split.map(|v| v.to_string()).collect::<Vec<String>>();

        match client.send_any_command(command, args).await {
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
