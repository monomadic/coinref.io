#[derive(Debug, Clone)]
pub struct CoinrefError {
    pub error_type: CoinrefErrorType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum CoinrefErrorType {
    ImportError,
    APIError,
    InsertRecordError,
    DatabaseConnectionError,
}

use std::error::Error;
use std::fmt::{self, Debug};
impl fmt::Display for CoinrefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.message, f)
    }
}

impl Error for CoinrefError {
    fn description(&self) -> &str { &*self.message }
}

use templar::parse::ParseError as TemplarError;
impl From<TemplarError> for CoinrefError {
    fn from(error: TemplarError) -> Self {
        CoinrefError {
            error_type: CoinrefErrorType::ImportError,
            message: templar_error(error),
        }
    }
}

impl From<::diesel::result::Error> for CoinrefError {
    fn from(error: ::diesel::result::Error) -> Self {
        CoinrefError {
            error_type: CoinrefErrorType::ImportError,
            message: format!("diesel error: {:?}", error),
        }
    }
}

impl From<::templar::output::WriteError<CoinrefError>> for CoinrefError {
    fn from(error: ::templar::output::WriteError<CoinrefError>) -> Self {

        match error {
            ::templar::output::WriteError::DirectiveError(directive) => directive,
            ::templar::output::WriteError::IO(io_error) => {
                CoinrefError {
                    error_type: CoinrefErrorType::ImportError,
                    message: format!("{}", io_error),
                }
            }
        }
    }
}

pub fn templar_error(error: TemplarError) -> String {
    let mut result:String = "Templar compilation error:\n".to_string();

    for (idx, c) in error.context.iter().enumerate() {
        let line_number = error.line_number + 2 + idx - error.context.len();
        let padded_line_number = format!("{:5}:", line_number);

        let marker = if error.line_number == line_number {
            ">"
        } else { " " };

        let line = format!("{}{} {}", marker, padded_line_number, c);
        result.push_str(&format!("{}", line));
    }

    result.push_str(&format!("reason -> {:?}\n", error.reason));

    result
}