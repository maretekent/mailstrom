
use std::convert::From;
use std::sync::mpsc::SendError;
use std::io::Error as IoError;
use email_format::rfc5322::ParseError;
use worker::Message;
use storage::MailstromStorageError;

#[derive(Debug)]
pub enum Error {
    Send(SendError<Message>),
    EmailParser(ParseError),
    General(String),
    Storage(String),
    DnsUnavailable,
    Lock,
    Io(IoError),
}

impl From<SendError<Message>> for Error {
    fn from(e: SendError<Message>) -> Error
    {
        Error::Send(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Error
    {
        Error::EmailParser(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error
    {
        Error::General(e)
    }
}

impl<S: MailstromStorageError> From<S> for Error {
    fn from(e: S) -> Error
    {
        Error::Storage(format!("{}", e))
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error
    {
        Error::Io(e)
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use ::std::error::Error as StdError;

        match *self {
            Error::Send(ref e) =>
                format!("{}: {:?}", self.description(), e).fmt(f),
            Error::EmailParser(ref e) =>
                format!("{}: {:?}", self.description(), e).fmt(f),
            Error::General(ref e) =>
                format!("{}: {}", self.description(), e).fmt(f),
            Error::Storage(ref s) =>
                format!("{}: {}", self.description(), s).fmt(f),
            _ => format!("{}", self.description()).fmt(f),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str
    {
        match *self {
            Error::Send(_) => "Unable to send message to worker",
            Error::EmailParser(_) => "Email does not parse",
            Error::General(_) => "General error",
            Error::Storage(_) => "Could not store or retrieve email state data",
            Error::DnsUnavailable => "DNS unavailable",
            Error::Lock => "Lock poisoned",
            Error::Io(_) => "I/O error",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error>
    {
        match *self {
            Error::Send(ref e) => Some(e),
            Error::EmailParser(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}
