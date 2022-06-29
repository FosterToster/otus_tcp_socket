use shtp::SHTPHandler;

pub struct ElectricSocket {}

impl SHTPHandler for ElectricSocket {
    fn on_request(&self, request: &shtp::SHTPRequest) -> shtp::SHTPResponse {
        shtp::SHTPResponse {
            result: true,
            data: format!("{} done", request.command),
        }
    }
}
