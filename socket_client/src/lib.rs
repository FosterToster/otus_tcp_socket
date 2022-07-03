pub const HOST: &str = "127.0.0.1";
pub const PORT: u16 = 6411;

pub mod client;


#[cfg(test)]
mod tests {
    use shtp::SHTPResponse;
    use crate::client;
    use crate::{HOST, PORT};

    fn get_client() -> client::SHTPElectricSocketClient {
        client::SHTPElectricSocketClient::new(HOST.to_string(), PORT)
    }

    fn observe_result(response: SHTPResponse) {
        if let Err(error) = response.observe() {
            panic!("{}", error)
        }
    }

    #[test]
    fn get_state() {
        observe_result(
            get_client().send_any_command("state", vec![]).unwrap()
        )
    }

    #[test]
    fn get_consumption() {
        observe_result(
            get_client().send_any_command("consumption", vec![]).unwrap()
        )
    }

    #[test]
    fn set_state_on() {
        observe_result(
            get_client().send_any_command("onoff", vec!["on".to_string()]).unwrap()
        )
    }
    
    #[test]
    fn set_state_off() {
        observe_result(
            get_client().send_any_command("onoff", vec!["off".to_string()]).unwrap()
        )
    }
    
    #[test]
    #[should_panic]
    fn set_not_state() {
        observe_result(
            get_client().send_any_command("onoff", vec!["bad_state".to_string()]).unwrap()
        )
    }
    
    #[test]
    #[should_panic]
    fn set_state_bad_args_count() {
        observe_result(
            get_client().send_any_command("onoff", vec!["bad_state_first".to_string(), "bad_state_second".to_string()]).unwrap()
        )
    }
    
    #[test]
    #[should_panic]
    fn set_state_no_args() {
        observe_result(
            get_client().send_any_command("onoff", vec![]).unwrap()
        )
    }

    #[test]
    #[should_panic]
    fn bad_command() {
        observe_result(
            get_client().send_any_command("bad_command", vec!["bad_state".to_string()]).unwrap()
        )
    }

}