mod about;
mod alert;
mod confirmation;
mod home;
mod record;

pub use about::about;
pub use alert::{AlertConfig, alert};
pub use confirmation::{ConfirmationConfig, confirmation};
pub use home::{Home, Message as HomeMessage};
pub use record::{Message as RecordMessage, Record};
