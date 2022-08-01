use rand::Rng;
use shtp::SHTPHandler;
use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ElectricSocketState {
    On,
    Off,
}

impl Display for ElectricSocketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl From<ElectricSocketState> for String {
    fn from(val: ElectricSocketState) -> Self {
        match val {
            ElectricSocketState::On => "on".to_string(),
            ElectricSocketState::Off => "off".to_string(),
        }
    }
}

pub struct ElectricSocket {
    state: Arc<Mutex<ElectricSocketState>>,
}

impl Default for ElectricSocket {
    fn default() -> Self {
        Self::new(ElectricSocketState::Off)
    }
}

impl ElectricSocket {
    fn new(state: ElectricSocketState) -> Self {
        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    fn onoff_state(&self, state: ElectricSocketState) -> ElectricSocketState {
        let mut lock = self.state.lock().unwrap();
        let prev_state = *lock;
        *lock = state;
        prev_state
    }

    fn switch_on(&self) -> ElectricSocketState {
        self.onoff_state(ElectricSocketState::On)
    }

    fn switch_off(&self) -> ElectricSocketState {
        self.onoff_state(ElectricSocketState::Off)
    }

    fn get_power_consumption_watt(&self) -> i32 {
        match self.get_state() {
            ElectricSocketState::On => rand::thread_rng().gen_range(0..1000),
            _ => 0,
        }
    }

    fn get_state(&self) -> ElectricSocketState {
        *self.state.lock().unwrap()
    }
}

#[async_trait::async_trait]
impl SHTPHandler for ElectricSocket {
    async fn on_request(&self, request: &shtp::SHTPRequest) -> shtp::SHTPResponse {
        match request.command.as_ref() {
            "onoff" => {
                if request.args.len() != 1 {
                    return shtp::SHTPResponse::fail(
                        "single argument 'state' is required for this command",
                    )
                    .await;
                }

                let previous_state = match request.args[0].as_ref() {
                    "on" => {
                        println!("switching state to 'on'");
                        self.switch_on()
                    }
                    "off" => {
                        println!("switching state to 'off'");
                        self.switch_off()
                    }
                    any => {
                        println!("unknown state");
                        return shtp::SHTPResponse::fail(
                            format!("state '{}' is not recognized", any).as_ref(),
                        )
                        .await;
                    }
                };

                return shtp::SHTPResponse::done(
                    format!(
                        "socket state switched from '{}' to '{}'",
                        previous_state,
                        self.get_state()
                    )
                    .as_ref(),
                )
                .await;
            }
            "consumption" => {
                println!("returning a power consumption");
                return shtp::SHTPResponse::done(
                    format!(
                        "current power consumption is {} watt",
                        self.get_power_consumption_watt()
                    )
                    .as_ref(),
                )
                .await;
            }
            "state" => {
                println!("returning current state");
                return shtp::SHTPResponse::done(
                    format!("current socket state is '{}'", self.get_state()).as_ref(),
                )
                .await;
            }
            _ => {
                println!("unknown command");
                return shtp::SHTPResponse::fail(
                    format!("unknown command '{}'", request.command).as_ref(),
                )
                .await;
            }
        }
    }
}
