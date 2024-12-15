use super::enums::ErrorType;

pub struct ErrorDetails {
    message: Option<String>,
    err_type: ErrorType,
    code: Option<isize>,
}

impl From<sqlite::Error> for ErrorDetails {
    fn from(sqlite_error: sqlite::Error) -> Self {
        Self {
            code: sqlite_error.code,
            err_type: ErrorType::Sqlite,
            message: sqlite_error.message,
        }
    }
}

impl From<std::io::Error> for ErrorDetails {
    fn from(io_error: std::io::Error) -> Self {
        Self {
            code: None,
            err_type: ErrorType::IO,
            message: Some(io_error.kind().to_string()),
        }
    }
}

impl From<nettle::Error> for ErrorDetails {
    fn from(nettle_error: nettle::Error) -> Self {
        Self {
            code: None,
            err_type: ErrorType::Nettle,
            message: Some(nettle_error.to_string()),
        }
    }
}

impl ErrorDetails {
    pub fn new(code: Option<isize>, message: Option<String>, err_type: ErrorType) -> Self {
        Self {
            message,
            err_type,
            code,
        }
    }
}
