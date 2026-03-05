use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::diagnostics::ui_error::UiError;
use crate::transport::client::Client;
use crate::transport::request::Request;

pub struct Controller {
    state: AppState,
    client: Client,
}

impl Controller {
    pub fn new(state: AppState, client: Client) -> Self {
        Self { state, client }
    }

    pub fn execute(&mut self, request: Request) -> Result<i32, UiError> {
        Reducer::apply(&mut self.state, Action::Started(request.summary()));
        match self.client.send(request) {
            Ok(response) => {
                Reducer::apply(&mut self.state, Action::Finished(response.exit_code));
                Ok(response.exit_code)
            }
            Err(e) => {
                Reducer::apply(&mut self.state, Action::Failed(e.to_string()));
                Err(e)
            }
        }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}
