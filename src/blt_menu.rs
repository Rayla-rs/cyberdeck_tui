use bluer::{Session, agent::Agent};
use bluetui::{app::AppResult, bluetooth::Controller, confirmation::PairingConfirmation};
/*
pub struct BltMenu {
    pub session: Arc<Session>,
    pub agent: Agent,
    pub controllers: Vec<Controller>,
}

impl BltMenu {
    pub async fn new() -> AppResult<Self> {
        let sesssion = Arc::new(bluer::Session::new().await?);

        let pairing_confimation = PairingConfirmation::new();

        let agent = Agent {
            request_default: false,
            request_confirmation: Some(Box::new(move |req| {
                request_confirmation(
                    req,
                    confirmation_display.clone(),
                    user_confirmation_receiver.clone(),
                    confirmation_message_sender.clone(),
                )
                .boxed()
            })),
            ..Default::default()
        };

        panic!()
    }
}
*/
