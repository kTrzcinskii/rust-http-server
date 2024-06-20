#[derive(Debug)]
pub enum ServerError {
    IncorrectRequestFormatError,
    WriteResponseError,
    FileReadingError,
}
