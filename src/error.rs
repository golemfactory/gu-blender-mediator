use failure::*;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Other(String),

    #[fail(display = "mailbox error {}", _0)]
    MailboxError(actix::MailboxError),
}

pub fn other(msg: &str) -> Error {
    Error::Other(msg.into())
}

impl From<actix::MailboxError> for Error {
    fn from(e: actix::MailboxError) -> Self {
        Error::MailboxError(e)
    }
}
