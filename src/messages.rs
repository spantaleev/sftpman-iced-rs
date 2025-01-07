use crate::application::ApplicationMessage;
use crate::pages::HomeMessage;
use crate::pages::RecordMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Application(ApplicationMessage),
    Home(HomeMessage),
    Record(RecordMessage),
}
