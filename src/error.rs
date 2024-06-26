#[derive(Debug)]
pub enum ServerError {
    IncorrectRequestFormatError,
    WriteResponseError,
    FileReadingError,
    IncorrectHttpMethodError,
    IncorrectHeaderError,
    IncorrectPathError,
    FileCreatingError,
    FileWritingError,
    TcpStreamReadingError,
    IncorrectRequestLineError,
    IncorrectEncodingError,
    EncodingError,
}
