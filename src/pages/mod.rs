mod about;
mod alert;
mod confirmation;
mod home;
mod record;

pub use about::about;
pub use alert::{alert, AlertConfig};
pub use confirmation::{confirmation, ConfirmationConfig};
pub use home::{Home, Message as HomeMessage};
pub use record::{Message as RecordMessage, Record};
