use std::fmt;

#[derive(Debug)]
pub enum Error {
    TeloxideRequest(teloxide::RequestError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TeloxideRequest(ref err) => {
                write!(f, "Telegram request error: {}", err)
            }
        }
    }
}

impl From<teloxide::RequestError> for Error {
    fn from(err: teloxide::RequestError) -> Self {
        Self::TeloxideRequest(err)
    }
}
