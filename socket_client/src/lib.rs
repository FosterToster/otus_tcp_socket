pub const HOST: &str = "127.0.0.1";
pub const PORT: u16 = 6411;

pub mod client;

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::{HOST, PORT};
    use shtp::SHTPResponse;

    async fn get_client() -> client::SHTPElectricSocketClient {
        client::SHTPElectricSocketClient::new(HOST.to_string(), PORT).await
    }

    fn observe_result(response: SHTPResponse) {
        if let Err(error) = response.observe() {
            panic!("{}", error)
        }
    }

    // #[test]
    #[tokio::test]
    async fn get_state() {
        observe_result(
            get_client()
                .await
                .send_any_command("state", vec![])
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn get_consumption() {
        observe_result(
            get_client()
                .await
                .send_any_command("consumption", vec![])
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn set_state_on() {
        observe_result(
            get_client()
                .await
                .send_any_command("onoff", vec!["on".to_string()])
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn set_state_off() {
        observe_result(
            get_client()
                .await
                .send_any_command("onoff", vec!["off".to_string()])
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    #[should_panic]
    async fn set_not_state() {
        observe_result(
            get_client()
                .await
                .send_any_command("onoff", vec!["bad_state".to_string()])
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    #[should_panic]
    async fn set_state_bad_args_count() {
        observe_result(
            get_client()
                .await
                .send_any_command(
                    "onoff",
                    vec![
                        "bad_state_first".to_string(),
                        "bad_state_second".to_string(),
                    ],
                )
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    #[should_panic]
    async fn set_state_no_args() {
        observe_result(
            get_client()
                .await
                .send_any_command("onoff", vec![])
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    #[should_panic]
    async fn bad_command() {
        observe_result(
            get_client()
                .await
                .send_any_command("bad_command", vec!["bad_state".to_string()])
                .await
                .unwrap(),
        )
    }
}
